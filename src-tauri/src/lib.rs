use tauri::{DragDropEvent, Emitter, Manager};

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![read_file])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::DragDrop(drag_event) = event {
                if let DragDropEvent::Drop { paths, .. } = drag_event {
                    if let Some(path) = paths.first() {
                        if let Some(path_str) = path.to_str() {
                            let _ = window.emit("file-opened", path_str);
                        }
                    }
                }
            }
        })
        .setup(|app| {
            // Handle file open from command line args
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = args[1].clone();
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.emit("file-opened", &file_path);
                    }
                });
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            // Handle macOS file open events (double-click on associated files)
            if let tauri::RunEvent::Opened { urls } = event {
                for url in urls {
                    if let Ok(path) = url.to_file_path() {
                        if let Some(path_str) = path.to_str() {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.emit("file-opened", path_str);
                            }
                        }
                    }
                }
            }
        });
}
