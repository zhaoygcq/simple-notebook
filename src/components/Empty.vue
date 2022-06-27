<script setup>
import { dialog } from '@tauri-apps/api';
import { ref } from 'vue';
import DialogVue from './Dialog.vue';
import { invoke } from '@tauri-apps/api';

const props = defineProps({
    showlist: Function
})

const showFolderDialog = async () => {
    let dirPath = await dialog.open({
      title: "请选择文件夹",
      defaultPath: "..",
      directory: true
    });

    // 发送请求给tauri，让tauri去读取文件
    let res = await invoke("read_folder", {
        event: dirPath
    });

    // 通知父组件，更新组件内容
    // emit2list()
    console.log(res, "======file checked=====");
}

const showCreateDialog = ref(false);

// 创建文件
const createDoc = () => {
    showCreateDialog.value = true;
}

const cancelCreate = () => {
    showCreateDialog.value = false;
}


const handleCreate = async (filename) => {
    try {
        console.log("msg=======", filename);
        // 向主进程发送消息，用于创建文件
        // 获取到文件创建的路径
        let res = await invoke('create_file', {
            event: filename
        })
        let data = {
            title: filename,
            createTime: +new Date()
        }
        // 文件创建成功，通知父组件更新组件内容
        emit2list({
            fileType: 0,
            data
        })
        cancelCreate();
    } catch(err) {

    }
}

const emit2list = (res) => {
    props.showlist(res);
}
</script>

<template>
    <div class="empty-container">
        <span>当前暂无文件，你可以选择</span>
        <span class="open-folder" @click="showFolderDialog">从文件夹导入</span>
        <span class="create" @click="createDoc">新建一个文件</span>
    </div>
    <DialogVue v-if="showCreateDialog" :confirm="handleCreate" :cancel="cancelCreate"/>
</template>

<style>
.empty-container {
    height: 100%;
    padding: 45vh 10px;
    font-size: 14px;
    display: flex;
    flex-direction: column;
}

.open-folder,
.create {
    color: lightblue;
    cursor: pointer;
}
</style>