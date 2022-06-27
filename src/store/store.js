import { Store } from 'tauri-plugin-store-api';

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