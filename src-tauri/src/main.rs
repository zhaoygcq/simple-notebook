#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    // 监听来自于渲染进程的数据通信
    .invoke_handler(tauri::generate_handler![
      cmd::get_md_in_folder
    ])
    .run(context)
    .expect("error while running tauri application");
}
