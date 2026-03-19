mod commands;
mod db;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use tauri::{
    AppHandle, Manager,
    tray::TrayIconBuilder,
    WebviewUrl, WebviewWindowBuilder,
};

static LAST_SHOW_TIME: AtomicU64 = AtomicU64::new(0);

fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

fn toggle_panel(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            LAST_SHOW_TIME.store(now_millis(), Ordering::SeqCst);
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else {
        create_main_window(app);
    }
}

fn create_main_window(app: &AppHandle) {
    LAST_SHOW_TIME.store(now_millis(), Ordering::SeqCst);
    let window = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Simple Note")
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

fn position_window_near_tray(app: &AppHandle, window: &tauri::WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }

            // Initialize database
            let app_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("simple-note.db");
            let database = db::Database::new(db_path.to_str().unwrap())
                .expect("Failed to initialize database");
            app.manage(database);

            // Create tray icon
            let app_handle = app.handle().clone();
            
            // Embed the 32x32 icon directly into the binary for reliability
            let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))?;

            TrayIconBuilder::with_id("main-tray")
                .icon(tray_icon)
                .icon_as_template(true)
                .tooltip("Simple Note")
                .on_tray_icon_event(move |_tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_panel(&app_handle);
                    }
                })
                .build(app)?;

            // Register global shortcut
            use tauri_plugin_global_shortcut::GlobalShortcutExt;
            let app_handle2 = app.handle().clone();
            app.global_shortcut().on_shortcut("CmdOrCtrl+Shift+N", move |_app, _shortcut, event| {
                if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                    toggle_panel(&app_handle2);
                }
            })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_group,
            commands::update_group,
            commands::delete_group,
            commands::get_all_groups,
            commands::create_note,
            commands::update_note,
            commands::delete_note,
            commands::get_all_notes,
            commands::copy_to_clipboard,
            commands::export_data,
            commands::import_data,
            commands::save_to_file,
            commands::read_file,
            commands::get_app_version,
            commands::quit_app,
            ])        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
