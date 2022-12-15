#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let _ = WindowBuilder::new(app, "main", WindowUrl::default())
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
