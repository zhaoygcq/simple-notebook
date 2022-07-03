use std::{fs, time::SystemTime, path::{self, PathBuf}};

use tauri::{command};
#[derive(Debug)]
pub struct FilesMsg {
    create_time: SystemTime,
    count: u64
}

#[command]
pub fn get_md_in_folder(event: String) -> Option<String> {
    println!("the path is {}", event);
    Some("test".to_string())
}

#[command]
pub fn create_file(filename:String, folderpath: String) -> Option<PathBuf> {
    handle_create_file(filename, folderpath)
}

#[command]
pub fn save_content(filepath: String, content: String) -> Option<String> {
    println!("filepath: {}======, content: {}====", filepath, content);
    fs::write(filepath, content).expect("write file error");
    Some("OK".to_string())
}

#[command]
pub fn get_content(filepath: String) -> Option<String> {
    println!("filepath: {}======", filepath);
    let res = fs::read_to_string(filepath).expect("");
    Some(res)
}


#[command]
// pub fn read_folder(event: String) -> Option<Vec<FilesMsg>> {
//     handle_read_folder(event)
// }

fn handle_create_file(filename: String, folderpath: String) -> Option<PathBuf> {
    let target_path = path::Path::new(&folderpath).join(filename + ".md");
    let file = fs::File::create(&target_path);

    match file {
        Ok(_) => {
            return Some(target_path)
        },
        Err(e) => {
            println!("error create===={}", e);
            return None;
        }
    }
}

fn handle_read_folder(path: String) -> Option<Vec<FilesMsg>> {
    let dirs = fs::read_dir(path);
    let mut res = vec![];
    match dirs {
        Ok(dir) => {
            for item in dir {
                if let Ok(entry) = item {
                    let meta_data = entry.metadata().expect("");
                    let create_time = meta_data.created().expect("");
                    res.push(FilesMsg {
                        create_time,
                        count: meta_data.len()
                    });
                }
            }

            println!("folder data is {:?}", res);
            return Some(res);
        },
        Err(_) => return None
    }
}