// Reorganised crate root – delegates logic to sub-modules for clarity.

pub mod commands;
pub mod config;
pub mod gitlab;
pub mod models;
pub mod state;
pub mod tray;

use commands::*;
use config::load_config;
use gitlab::fetch_all_releases;
use state::AppState;
use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, ShortcutState};
use tray::{install_tray, update_tray_icon};

use std::sync::Arc;
use tauri::{Emitter, Manager, WindowEvent};
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_releases,
            refresh_releases,
            open_release_url,
            mark_releases_as_seen,
            show_main_window,
            hide_main_window
        ])
        .setup(|app| {
            // Load configuration
            let config = load_config().map_err(|e| {
                eprintln!("Failed to load configuration: {}", e);
                eprintln!("Please create a .env file with GITLAB_API_TOKEN and GITLAB_PROJECTS");
                e
            })?;

            // HTTP client
            let client = reqwest::Client::new();

            // Application state
            let state = AppState {
                config,
                releases: Arc::new(Mutex::new(Vec::new())),
                previous_releases: Arc::new(Mutex::new(Vec::new())),
                client,
                has_new_releases: Arc::new(Mutex::new(false)),
            };
            app.manage(state);

            // Tray installation
            install_tray(app.handle())?;

            // Register global shortcut CTRL+ALT+G to toggle the window visibility
            {
                let app_handle_toggle = app.handle();
                use tauri::Manager;

                app_handle_toggle.plugin(
                    ShortcutBuilder::new()
                        .with_shortcuts(["Alt+G"])?
                        .with_handler(move |app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let windows = app.webview_windows();
                                if let Some(window) = windows.values().next() {
                                    if window.is_visible().unwrap_or(false) {
                                        let _ = window.hide();
                                    } else {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                            }
                        })
                        .build(),
                )?;
            }

            // Set up window close handler to hide to tray instead of closing
            let windows = app.webview_windows();
            if let Some(window) = windows.values().next() {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                        println!("Window hidden to tray via close button");
                    }
                });
            }

            // Background refresh task
            let app_handle_bg = app.handle().clone();
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                println!("Starting GitLab releases background task...");

                if let Some(state) = app_handle_bg.try_state::<AppState>() {
                    if let Ok(initial_releases) = fetch_all_releases(&state).await {
                        let mut prev = state.previous_releases.lock().await;
                        *prev = initial_releases.clone();
                        drop(prev);

                        let mut releases = state.releases.lock().await;
                        *releases = initial_releases.clone();
                        drop(releases);

                        let _ = app_handle_bg.emit("releases-loaded", &initial_releases);
                    }
                }

                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300));
                interval.tick().await;

                loop {
                    interval.tick().await;
                    println!("Auto-refreshing GitLab releases...");

                    if let Some(state) = app_handle_bg.try_state::<AppState>() {
                        if let Ok(new_releases) = fetch_all_releases(&state).await {
                            let previous_releases = state.previous_releases.lock().await;
                            let new_items =
                                gitlab::detect_new_releases(&new_releases, &previous_releases);
                            drop(previous_releases);

                            let mut prev = state.previous_releases.lock().await;
                            *prev = new_releases.clone();
                            drop(prev);

                            let mut releases = state.releases.lock().await;
                            *releases = new_releases.clone();
                            drop(releases);

                            if !new_items.is_empty() {
                                let mut has_new = state.has_new_releases.lock().await;
                                *has_new = true;
                                drop(has_new);
                                let _ = update_tray_icon(&app_handle_bg, true);
                            }

                            let _ = app_handle_bg.emit("releases-updated", &new_releases);
                        }
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
