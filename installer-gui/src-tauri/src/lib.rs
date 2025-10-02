use tauri::Emitter;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

#[tauri::command]
async fn install_rayhunter(app_handle: tauri::AppHandle, args: String) {
    let (mut rx, _child) = app_handle
        .shell()
        .sidecar("installer-cli")
        .unwrap()
        .args(args.split_whitespace())
        .spawn()
        .unwrap();
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line_bytes) | CommandEvent::Stderr(line_bytes) => {
                let line = String::from_utf8(line_bytes).unwrap();
                app_handle.emit("installer-output", &line).unwrap();
            }
            _ => (),
        };
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![install_rayhunter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
