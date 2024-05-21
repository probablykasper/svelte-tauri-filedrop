#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WebviewWindowBuilder, WebviewUrl};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("Tauri Template")
        .inner_size(800.0, 600.0)
        .min_inner_size(400.0, 200.0)
        .fullscreen(false)
        .build()
        .expect("Unable to create window");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
