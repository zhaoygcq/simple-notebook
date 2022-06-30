import { invoke } from "@tauri-apps/api";

export async function createFileApi(filename, folderpath) {
    try {
        let res = await invoke('create_file', {
            filename,
            folderpath
        })

        return res;
    } catch(e) {
        return Promise.reject(e);
    }
}


export async function readFolderApi(dirpath) {
    try {
        let res = await invoke("read_folder", {
            event: dirPath
        });

        return res;
    }catch(e) {
        return Promise.reject(e);
    }
}

export async function saveContentApi(filepath, content) {
    try {
        let res = await invoke("save_content", {
            filepath,
            content
        });

        return res;
    } catch(e) {
        return Promise.reject(e);
    }
}

export async function getContentApi(filepath) {
    try {
        let res = await invoke("get_content", {
            filepath
        });

        return res;
    }catch(e) {
        return Promise.reject(e);
    }
}