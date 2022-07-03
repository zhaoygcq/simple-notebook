import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";

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

export async function listenDoForFileApi(eventMap) {
    try {
        await listen('do_for_file', async ({event, payload}) => {
            if(eventMap[payload]) await eventMap[payload]();
            console.log("listen=======", payload, eventMap);
        });
    } catch(e) {
        console.log("listen do for file error", e);
    }
}