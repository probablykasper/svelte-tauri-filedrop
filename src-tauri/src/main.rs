#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

fn main() {
  tauri::Builder::default()
    .create_window("Yo", WindowUrl::default(), |win, webview| {
      let win = win
        .title("Tauri Template")
        .resizable(true)
        .decorations(true)
        .inner_size(800.0, 600.0)
        .min_inner_size(400.0, 200.0)
        .skip_taskbar(false)
        .fullscreen(false);
      return (win, webview);
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
