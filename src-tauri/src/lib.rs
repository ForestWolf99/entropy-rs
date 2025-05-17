use crate::commands::{
    add_count, debug, generate_output, hello_world, setup, CharacterLists, Counter,
};

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Webview
        )).build())
        .manage(Counter::default())
        .manage(CharacterLists::default())
        .invoke_handler(tauri::generate_handler![
            hello_world,
            add_count,
            generate_output,
            debug,
            setup
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
