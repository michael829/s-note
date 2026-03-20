use crate::db::Database;
use crate::panel::toggle_panel;
use tauri::{App, Manager, Runtime};
use tauri::tray::TrayIconBuilder;

pub fn setup<R: Runtime>(app: &mut App<R>) -> Result<(), Box<dyn std::error::Error>> {
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

    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("s-note.db");
    let database = Database::new(db_path.to_str().unwrap())
        .expect("Failed to initialize database");
    app.manage(database);

    let app_handle = app.handle().clone();
    let tray_icon = tauri::image::Image::from_bytes(include_bytes!("../icons/32x32.png"))?;

    TrayIconBuilder::with_id("main-tray")
        .icon(tray_icon)
        .icon_as_template(false)
        .tooltip("S-Note")
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

    use tauri_plugin_global_shortcut::GlobalShortcutExt;
    let app_handle2 = app.handle().clone();
    app.global_shortcut().on_shortcut("CmdOrCtrl+Shift+N", move |_app, _shortcut, event| {
        if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
            toggle_panel(&app_handle2);
        }
    }).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

    Ok(())
}
