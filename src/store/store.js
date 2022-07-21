import { Store } from 'tauri-plugin-store-api';
export const StoreKey = "list";
const store = new Store('.settings.dat');

export async function setData(data) {
    let { key, val } = data;
    try {
      await store.set(key, val);
    } catch(e) {
        return Promise.reject(e);
    }
}

export async function getData(key) {
    try {
       let res = await store.get(key);
       return res;
    } catch(e) {
        return Promise.reject(e);
    }
}

/**
 * 
 * @param {缓存中的键值} key 
 * @param {一个路径信息} filePath 
 * @returns 
 */
export async function removeItem(key, filePath) {
    try {
        let res = await store.get(key);
        if(res.length) {
            res = res.filter(item => item.filePath != filePath);
        }
        return res;
     } catch(e) {
        return Promise.reject(e);
     }
}