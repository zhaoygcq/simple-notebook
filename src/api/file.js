import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { handleFolderRes } from "../utils";

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

export async function readFolderApi(dirPath) {
    try {
        let res = await invoke("read_folder", {
            event: dirPath
        });
        // 这里的响应值为一个对象数组，对象数据格式为
        /**
         * count: number,
         * file_path: 完全路径,
         * update_time: 修改时间
         */
        return handleFolderRes(res);
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
            try {
                if(eventMap[payload]) await eventMap[payload]();    
            } catch(e) {
                console.log("handle error=======", e);
            }
        });
    } catch(e) {
        console.log("listen do for file error", e);
    }
}