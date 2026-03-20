use crate::app_state::{IGNORE_BLUR, LAST_SHOW_TIME};
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn toggle_panel<R: Runtime>(app: &AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            LAST_SHOW_TIME.store(now_millis(), Ordering::SeqCst);
            let _ = window.show();
            let _ = window.set_focus();
            let _ = position_window_near_tray(app, &window);
        }
    } else {
        create_main_window(app);
    }
}

fn create_main_window<R: Runtime>(app: &AppHandle<R>) {
    LAST_SHOW_TIME.store(now_millis(), Ordering::SeqCst);
    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("S-Note")
        .inner_size(360.0, 500.0)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .background_color(tauri::window::Color(0, 0, 0, 0))
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(true)
        .focused(true)
        .build();

    if let Ok(win) = window {
        let _ = position_window_near_tray(app, &win);

        let win_clone = win.clone();
        win.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                if IGNORE_BLUR.load(Ordering::SeqCst) {
                    return;
                }
                if now_millis() - LAST_SHOW_TIME.load(Ordering::SeqCst) < 300 {
                    return;
                }
                for (label, win) in win_clone.app_handle().webview_windows() {
                    if (label.starts_with("editor") || label.starts_with("settings"))
                        && win.is_visible().unwrap_or(false)
                    {
                        return;
                    }
                }
                let _ = win_clone.hide();
            }
        });
    }
}

fn position_window_near_tray<R: Runtime>(
    app: &AppHandle<R>,
    window: &WebviewWindow<R>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(tray) = app.tray_by_id("main-tray") {
        if let Some(rect) = tray.rect().ok().flatten() {
            let (tray_x, tray_y) = match rect.position {
                tauri::Position::Logical(pos) => (pos.x, pos.y),
                tauri::Position::Physical(pos) => {
                    let scale = window.scale_factor()?;
                    (pos.x as f64 / scale, pos.y as f64 / scale)
                }
            };
            let tray_height = match rect.size {
                tauri::Size::Logical(s) => s.height,
                tauri::Size::Physical(s) => {
                    let scale = window.scale_factor()?;
                    s.height as f64 / scale
                }
            };

            let window_size = window.outer_size()?;
            let scale = window.scale_factor()?;
            let window_width = window_size.width as f64 / scale;

            let x = tray_x - (window_width / 2.0);
            let y = tray_y + tray_height + 4.0;

            window.set_position(tauri::Position::Logical(tauri::LogicalPosition::new(x, y)))?;
        }
    }
    Ok(())
}
