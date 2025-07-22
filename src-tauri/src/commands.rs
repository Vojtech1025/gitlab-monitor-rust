use crate::{
    gitlab::{detect_new_releases, fetch_all_releases},
    state::AppState,
    tray::update_tray_icon,
};
use tauri::{Manager, Runtime};

#[tauri::command]
pub async fn get_releases(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<crate::models::GitLabRelease>, String> {
    let releases = state.releases.lock().await;
    Ok(releases.clone())
}

#[tauri::command]
pub async fn refresh_releases(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<Vec<crate::models::GitLabRelease>, String> {
    match fetch_all_releases(&state).await {
        Ok(new_releases) => {
            let previous_releases = state.previous_releases.lock().await;
            let new_items = detect_new_releases(&new_releases, &previous_releases);
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
                let _ = update_tray_icon(&app, true);
            }
            Ok(new_releases)
        }
        Err(e) => Err(format!("Failed to refresh releases: {}", e)),
    }
}

#[tauri::command]
pub async fn open_release_url(url: String) -> Result<(), String> {
    if let Err(e) = open::that(&url) {
        return Err(format!("Failed to open URL: {}", e));
    }
    Ok(())
}

#[tauri::command]
pub async fn mark_releases_as_seen(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut has_new = state.has_new_releases.lock().await;
    *has_new = false;
    Ok(())
}

#[tauri::command]
pub fn show_main_window<R: Runtime>(app: tauri::AppHandle<R>) {
    let windows = app.webview_windows();
    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .set_focus()
        .expect("Can't Bring Window to Focus");
    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .show()
        .expect("Can't Show Window");
}

#[tauri::command]
pub fn hide_main_window<R: Runtime>(app: tauri::AppHandle<R>) {
    let windows = app.webview_windows();
    windows
        .values()
        .next()
        .expect("Sorry, no window found")
        .hide()
        .expect("Can't Hide Window");
}
