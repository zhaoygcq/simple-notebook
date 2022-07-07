<script setup>
const emit = defineEmits([
    'showlist',
    'createFile',
    'openFolder'
])

const openFolderDialog = async () => {
    emit('openFolder');
}


// 创建文件
const createDoc = () => {
    emit('createFile');
}

</script>

<template>
    <div class="empty-container">
        <span>当前暂无文件，你可以选择</span>
        <span class="open-folder" @click="openFolderDialog">从文件夹导入</span>
        <span class="create" @click="createDoc">新建一个文件</span>
    </div>
    <!-- <DialogVue v-if="showCreateDialog" :confirm="handleCreate" :cancel="cancelCreate"/> -->
</template>

<style>
.empty-container {
    font-size: 14px;
    display: flex;
    flex-direction: column;
    position: absolute;
    top: 50%;
    translate: 10% -50%;
}

.open-folder,
.create {
    color: lightblue;
    cursor: pointer;
}
</style>