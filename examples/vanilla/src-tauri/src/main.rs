#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

fn main() {
  let port = portpicker::pick_unused_port().expect("failed to find unused port");
  tauri::Builder::default()
    .plugin(tauri_plugin_localhost::Localhost::new(port))
    .setup(move |app| {
      app
        .create_window(
          "main",
          WindowUrl::External(format!("http://localhost:{}", port).parse().unwrap()),
          |window_builder, webview_attributes| {
            (
              window_builder.title("Localhost Example"),
              webview_attributes,
            )
          },
        )
        .unwrap();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
