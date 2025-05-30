use crate::commands::{
    generate_output, setup, CharacterLists,
};

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Webview
        )).build())
        .manage(CharacterLists::default())
        .invoke_handler(tauri::generate_handler![
            generate_output,
            setup
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
