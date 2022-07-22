<script setup>
import { onMounted, reactive, ref } from 'vue';
import ListItemVue from '../components/ListItem.vue';
import EmptyForList from "../components/EmptyForList.vue";
import { setData, getData, StoreKey } from "../store/store";
import DialogVue from "../components/Dialog.vue";
import { dialog } from '@tauri-apps/api';
import { createFileApi, readFolderApi, listenDoForFileApi } from "../api/file";

let emit = defineEmits([
  'itemclick'
]);

// 向tauri查询文件列表
const state = reactive({
  list: []
});
const listDom = ref(null);
const showCreateDialog = ref(false);
const checkedItem = ref(null);

// 点击列表项，需要展示列表所对应的文件都内容
const showContent = (target) => {
  checkedItem.value = target;
  emit('itemclick', target.filePath);
}

// 用于弹出表单，填写创建文件的信息
const handleListClick = (evt) => {
  if(!evt || evt.target === listDom.value) {
    showCreateDialog.value = true
  }
}

// 重置列表信息
const updateForce = async (data) => {
  state.list = data || [];
  checkedItem.value = null;
  await setData({key: StoreKey, val: [...state.list]});
}


defineExpose({updateForce})

// 更新列表信息，并写入缓存
const updateList = async (res) => {
  console.log(res, "======updateList=======");
  let store = await getData(StoreKey);
  let currentData = null;
  if(Array.isArray(res)) {
    currentData = res;
  } else {
    currentData = [ res.data ];
  }

  // 设置当前新建文本为选中项
  showContent(res.data);
  state.list = [ ...currentData, ...state.list];
  if(Array.isArray(store)) {
    await setData({key: StoreKey, val: [...currentData, ...store]});
  } else {
    await setData({key: StoreKey, val: [...currentData]});  
  }
}

// 创建一个文本文件
const handleCreate = async ({filename, folderpath}) => {
    try {
        // 向主进程发送消息，用于创建文件
        // 获取到文件创建的路径
        let res = await createFileApi(filename, folderpath);

        let data = {
            title: filename,
            createTime: +new Date(),
            filePath: res
        }
        // 文件创建成功，通知父组件更新组件内容
        updateList({
            fileType: 0,
            data
        })
        cancelCreate();
    } catch(err) {

    }
}

const cancelCreate = () => {
    showCreateDialog.value = false;
}

// 清空当前工作区域的展示内容
const emptyWorkspace = async () => {
  await setData({key: StoreKey, val: []});
  state.list = [];
  // 清空文本区域的现实内容
    emit('itemclick', '');
}

// 打开文件夹
const showFolderDialog = async () => {
    let dirPath = await dialog.open({
      title: "请选择文件夹",
      defaultPath: "..",
      directory: true
    });
    console.log("=======open-folder=====", dirPath)
    // 发送请求给tauri，让tauri去读取文件
    let res = await readFolderApi(dirPath);
    // 清空当前的工作区
    await emptyWorkspace();
    await updateList(res);
    // 更新列表项
    console.log(res, "======file checked=====");
}

// 针对主进程发过来的信息作出的响应
const ListenResEventMap = {
  open: showFolderDialog,
  create: handleListClick,
  empty: emptyWorkspace
}


onMounted(async() => {
  try {
    let store = await getData(StoreKey);
    if(store) state.list = store;
    listenDoForFileApi(ListenResEventMap);
  } catch(e) {
    console.log(e, "======list vue mounted");
  }
})
</script>

<template>
  <div id="list-container" ref="listDom" @dblclick="handleListClick">
    <ListItemVue
      v-for="item in state.list"
      :class="checkedItem && item.filePath === checkedItem.filePath ? 'checked' : ''"
      :key="item"
      :title="item.title"
      :create-time="item.createTime"
      :count="item.count"
      :desc="item.desc"
      @click="showContent(item)"
    ></ListItemVue>
    <EmptyForList
      v-if="!state.list.length"
      @showlist="updateList"
      @create-file="handleListClick"
      @open-folder="showFolderDialog"
    />
    <DialogVue v-if="showCreateDialog" :confirm="handleCreate" :cancel="cancelCreate"/>
  </div>
</template>

<style scoped>
#list-container {
  height: 100%;
  overflow-y: auto;
}

.checked {
  color: lightcoral;
  border-color: lightcoral;
}
</style>