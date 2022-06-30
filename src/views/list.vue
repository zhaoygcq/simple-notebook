<script setup>
import { onMounted, reactive, ref } from 'vue';
import ListItemVue from '../components/ListItem.vue';
import EmptyVue from "../components/Empty.vue";
import { dialog } from "@tauri-apps/api";
import { setData, getData } from "../store/store";
import DialogVue from "../components/Dialog.vue";
import { createFileApi } from "../api/file";

let emit = defineEmits(['itemclick']);

// 向tauri查询文件列表
const list = reactive([]);
const listDom = ref(null);
const showCreateDialog = ref(false);
const checkedItem = ref(null);

const showContent = (target) => {
  checkedItem.value = target;
  emit('itemclick', target.filePath || "");
}

const handleListClick = (evt) => {
  if(evt.target === listDom.value) {
    showCreateDialog.value = true
  }
}

const handlelistShow = async (res) => {
  list.push(res.data);
  let store = await getData("list");
  if(Array.isArray(store)) {
    await setData({key: "list", val: [...store, res.data]})
  } else {
    await setData({key: "list", val: [res.data]})  
  }
}

onMounted(async() => {
  try {
    let store = await getData("list");
    if(store) store.map(item => list.push(item));
  } catch(e) {
    console.log(e, "======list vue mounted");
  }
})

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
        handlelistShow({
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

</script>

<template>
  <div id="list-container" ref="listDom" @dblclick="handleListClick">
    <ListItemVue
      v-for="item in list"
      :class="item === checkedItem ? 'checked' : ''"
      :key="item"
      :title="item.title"
      :create-time="item.createTime"
      :count="item.count"
      :desc="item.desc"
      @click="showContent(item)"
    ></ListItemVue>
    <EmptyVue v-if="!list.length" @showlist="handlelistShow"/>
    <DialogVue v-if="showCreateDialog" :confirm="handleCreate" :cancel="cancelCreate"/>
  </div>
</template>

<style scoped>
#list-container {
  height: 100%;
}

.checked {
  color: lightcoral;
  border-color: lightcoral;
}
</style>