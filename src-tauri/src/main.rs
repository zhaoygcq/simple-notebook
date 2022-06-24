#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use tauri::{
  CustomMenuItem,
  Menu,
  MenuItem,
  Submenu,
  AboutMetadata
};

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(get_menu())
    // 监听来自于渲染进程的数据通信
    .invoke_handler(tauri::generate_handler![
      cmd::get_md_in_folder
    ])
    .run(context)
    .expect("error while running tauri application");
}


// 自定义菜单栏
pub fn get_menu() -> Menu {
  let authors = vec!["zhao".to_string()];
  let mut aboutmetadata = AboutMetadata::new();
  aboutmetadata = aboutmetadata.version("0.0.1");
  aboutmetadata = aboutmetadata.authors(authors);
  aboutmetadata = aboutmetadata.comments("test".to_string());
  aboutmetadata = aboutmetadata.copyright("cq".to_string());
  aboutmetadata = aboutmetadata.license("MIT".to_string());
  
  // 创建自定义的菜单项
  #[allow(unused_mut)]
  let mut create_item = CustomMenuItem::new("create", "新建文件").accelerator("CmdOrControl+N");
    
  let my_app_menu = Menu::new()
  .add_native_item(MenuItem::About(
    "Simple Note".to_string(),
    aboutmetadata
  ));

  let test_menu = Menu::new()
    .add_item(CustomMenuItem::new(
      "open",
      "打开文件夹",
    ))
    .add_native_item(MenuItem::Separator)
    .add_item(create_item);

  // add all our childs to the menu (order is how they'll appear)
  Menu::new()
    .add_submenu(Submenu::new("", my_app_menu)) // 第一个菜单项代表当前应用，这里的title字段无效
    .add_submenu(Submenu::new("编辑", test_menu))
}
