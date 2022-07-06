#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

use tauri_plugin_store::PluginBuilder;

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
    .on_menu_event(|event| {
      let menu_id = event.menu_item_id();
      event.window().emit("do_for_file", menu_id).expect("test");
      // 自定义菜单的点击事件
      println!("你刚才点击了:{:?}", event.menu_item_id());

    })
    // 监听来自于渲染进程的数据通信
    .invoke_handler(tauri::generate_handler![
      cmd::get_md_in_folder,
      cmd::create_file,
      cmd::save_content,
      cmd::get_content,
      cmd::read_folder
    ])
    .plugin(PluginBuilder::default().build())
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
  let mut create_item = CustomMenuItem::new("create", "New File").accelerator("CmdOrControl+N");
  let mut hide_sidebar: CustomMenuItem = CustomMenuItem::new("hide_sidebar", "Hide/Show the sidebar");
  let mut open_folder = CustomMenuItem::new("open", "Open Folder").accelerator("CmdOrControl+F");
  let mut empty_workspace = CustomMenuItem::new("empty", "Remove workspace").accelerator("CmdOrControl+R");
  let my_app_menu = Menu::new()
  .add_native_item(MenuItem::About(
    "Simple Note".to_string(),
    aboutmetadata
  ));

  let file_menu = Menu::new()
    .add_item(open_folder)
    .add_native_item(MenuItem::Separator)
    .add_item(create_item)
    .add_native_item(MenuItem::Separator)
    .add_item(empty_workspace);

  let edit_menu = Menu::new()
    .add_native_item(MenuItem::Undo)
    .add_native_item(MenuItem::Redo)
    .add_native_item(MenuItem::Separator)
    .add_native_item(MenuItem::Cut)
    .add_native_item(MenuItem::Copy)
    .add_native_item(MenuItem::Paste)
    .add_native_item(MenuItem::SelectAll);

  let window_menu = Menu::new()
    .add_item(hide_sidebar)
    .add_native_item(MenuItem::Minimize)
    .add_native_item(MenuItem::Zoom)
    .add_native_item(MenuItem::Hide)
    .add_native_item(MenuItem::Quit);

  // add all our childs to the menu (order is how they'll appear)
  Menu::new()
    .add_submenu(Submenu::new("", my_app_menu)) // 第一个菜单项代表当前应用，这里的title字段无效
    .add_submenu(Submenu::new("File", file_menu))
    .add_submenu(Submenu::new("Edit", edit_menu))
    .add_submenu(Submenu::new("Window", window_menu))
}