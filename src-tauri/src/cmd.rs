use std::{fs, ffi::OsString};

use tauri::{command};

#[command]
pub fn get_md_in_folder(event: String) -> Option<String> {
    println!("the path is {}", event);
    Some("test".to_string())
}

fn create_file(path: String) -> Option<String> {
    let file = fs::File::create(path);

    match file {
        Ok(_) => {
            return Some("OK".to_string())
        },
        Err(_) => {
            return None;
        }
    }
}

fn read_folder(path: String) -> Option<Vec<OsString>> {
    let dirs = fs::read_dir(path);
    let mut res = vec![];
    match dirs {
        Ok(dir) => {
            for item in dir {
                match item {
                    Ok(entry) => res.push(entry.file_name()),
                    Err(_) => {}
                }
            }

            return Some(res);
        },
        Err(_) => return None
    }
}