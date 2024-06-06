// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let resource_dir = app.path_resolver().resolve_resource("../server-executable")
            .unwrap();
  
      
            // 确保路径存在并且是一个文件
            if !resource_dir.exists() || !resource_dir.is_file() {
              Err(format!(
                "无法找到 server-executable。(路径：{:?})",
                resource_dir
              ))?;
            }
      
            // 启动 server-executable
            if let Err(e) = Command::new(resource_dir)
              .spawn()
            {
              Err(format!("Failed to start server-executable: {}", e))?;
            }
      
            // 正确启动 server-executable 后返回 Ok(())
            Ok(())
          })

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
