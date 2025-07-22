use once_cell::sync::Lazy;
use std::io::Cursor;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

use crate::state::AppState;

// Static tray icons
static TRAY_ICON_NORMAL: Lazy<tauri::image::Image<'static>> =
    Lazy::new(|| load_png_icon(include_bytes!("../icons/tray-icon-32.png")));
static TRAY_ICON_BLUE: Lazy<tauri::image::Image<'static>> =
    Lazy::new(|| load_png_icon(include_bytes!("../icons/tray-icon-blue-32.png")));

fn load_png_icon(data: &[u8]) -> tauri::image::Image<'static> {
    let decoder = png::Decoder::new(Cursor::new(data));
    let mut reader = decoder.read_info().expect("Invalid PNG");
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).expect("Decode frame");
    let bytes = &buf[..info.buffer_size()];
    tauri::image::Image::new_owned(bytes.to_vec(), info.width, info.height)
}

pub fn create_tray_menu<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    let show_item = MenuItem::with_id(app, "show", "Show GitLab Releases", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item])?;
    Ok(menu)
}

pub fn update_tray_icon<R: Runtime>(
    app: &tauri::AppHandle<R>,
    has_notifications: bool,
) -> tauri::Result<()> {
    let tooltip = if has_notifications {
        "🔵 GitLab Releases Monitor - New Releases Available!"
    } else {
        "GitLab Releases Monitor"
    };

    if let Some(tray) = app.tray_by_id("gitlab-monitor-tray") {
        let _ = tray.set_tooltip(Some(tooltip));
        let icon = if has_notifications {
            &*TRAY_ICON_BLUE
        } else {
            &*TRAY_ICON_NORMAL
        };
        let _ = tray.set_icon(Some(icon.clone()));
    }
    Ok(())
}

// Event handlers
pub fn handle_tray_event<R: Runtime>(app: &tauri::AppHandle<R>, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click { button, .. } => match button {
            MouseButton::Left => {
                let windows = app.webview_windows();
                if let Some(window) = windows.values().next() {
                    let _ = window.show();
                    let _ = window.set_focus();

                    if let Some(state) = app.try_state::<AppState>() {
                        let has_new_arc = state.has_new_releases.clone();
                        let app_handle_clone = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let mut has_new = has_new_arc.lock().await;
                            if *has_new {
                                *has_new = false;
                                let _ = update_tray_icon(&app_handle_clone, false);
                            }
                        });
                    }
                }
            }
            MouseButton::Right => {}
            _ => {}
        },
        TrayIconEvent::DoubleClick { .. } => {
            let windows = app.webview_windows();
            if let Some(window) = windows.values().next() {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    }
}

pub fn handle_tray_menu_event<R: Runtime>(
    app: &tauri::AppHandle<R>,
    event: tauri::menu::MenuEvent,
) {
    match event.id().as_ref() {
        "show" => {
            let windows = app.webview_windows();
            if let Some(window) = windows.values().next() {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

pub fn install_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let tray_menu = create_tray_menu(app)?;
    TrayIconBuilder::with_id("gitlab-monitor-tray")
        .tooltip("GitLab Releases Monitor")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&tray_menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
            handle_tray_event(tray.app_handle(), event);
        })
        .on_menu_event(|tray, event| {
            handle_tray_menu_event(tray.app_handle(), event);
        })
        .build(app)?;
    Ok(())
}
