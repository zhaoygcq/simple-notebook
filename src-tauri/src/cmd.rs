use std::{fs, ffi::OsString};

use tauri::{command};

#[command]
pub fn get_md_in_folder(event: String) -> Option<String> {
    println!("the path is {}", event);
    Some("test".to_string())
}

#[command]
pub fn create_file(event: String) -> Option<String> {
    handle_create_file(event)
}

#[command]
pub fn read_folder(event: String) -> Option<Vec<OsString>> {
    handle_read_folder(event)
}

fn handle_create_file(path: String) -> Option<String> {
    let file = fs::File::create(path + ".md");

    match file {
        Ok(_) => {
            return Some("OK".to_string())
        },
        Err(_) => {
            return None;
        }
    }
}

fn handle_read_folder(path: String) -> Option<Vec<OsString>> {
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