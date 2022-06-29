#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use std::{fs::File, env::current_dir};

use tauri::{
  CustomMenuItem,
  Menu,
  MenuItem,
  Submenu,
  AboutMetadata,
  api::{dialog::FileDialogBuilder, path::app_dir}
};

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(get_menu())
    .on_menu_event(|event| {
      let menu_id = event.menu_item_id();
      if menu_id == "create" {
        cmd::create_file("Untitled".to_string(), "".to_string());
      } else if menu_id == "open" {
        // 主进程打开文件选择窗口
        FileDialogBuilder::new().pick_folder(|folder_path| {
          // do something with the optional folder path here
          // the folder path is `None` if the user closed the dialog
          if let Some(target) = folder_path {
            println!("folder path is {:?}", target);
            // cmd::read_folder(target.to_str().expect(" ").to_string());
          }
        })
      }
      // 自定义菜单的点击事件
      println!("你刚才点击了:{:?}", event.menu_item_id());
    })
    // 监听来自于渲染进程的数据通信
    .invoke_handler(tauri::generate_handler![
      cmd::get_md_in_folder,
      cmd::create_file,
      cmd::save_content
      // cmd::read_folder
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

  let file_menu = Menu::new()
    .add_item(CustomMenuItem::new(
      "open",
      "打开文件夹",
    ).accelerator("CmdOrControl+F"))
    .add_native_item(MenuItem::Separator)
    .add_item(create_item);

  let edit_menu = Menu::new()
    .add_native_item(MenuItem::Undo)
    .add_native_item(MenuItem::Redo);

  let window_menu = Menu::new()
    .add_native_item(MenuItem::Minimize)
    .add_native_item(MenuItem::Zoom)
    .add_native_item(MenuItem::Hide)
    .add_native_item(MenuItem::Quit);

  // add all our childs to the menu (order is how they'll appear)
  Menu::new()
    .add_submenu(Submenu::new("", my_app_menu)) // 第一个菜单项代表当前应用，这里的title字段无效
    .add_submenu(Submenu::new("文件", file_menu))
    .add_submenu(Submenu::new("编辑", edit_menu))
    .add_submenu(Submenu::new("窗口", window_menu))
}
