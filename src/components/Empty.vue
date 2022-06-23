<script setup>
import { dialog } from '@tauri-apps/api';
import { ref } from 'vue';
import DialogVue from './Dialog.vue';

const showFolderDialog = async () => {
    let res = await dialog.open({
      title: "请选择文件夹",
      defaultPath: "..",
      directory: true
    });

    // 发送请求给tauri，让tauri去读取文件

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


const handleCreate = (filename) => {
    console.log("msg=======", filename);
    cancelCreate();
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