use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.setup(|app| {
			let _ = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
				.title("Tauri Template")
				.inner_size(1200.0, 900.0)
				.min_inner_size(400.0, 200.0)
				.fullscreen(false)
				.build()
				.expect("Unable to create window");
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
