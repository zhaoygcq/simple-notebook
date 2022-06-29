<script setup>
import { onMounted, reactive } from 'vue';
import ListItemVue from '../components/ListItem.vue';
import EmptyVue from "../components/Empty.vue"
import { invoke, event } from "@tauri-apps/api";
import { setData, getData } from "../store/store";

let emit = defineEmits(['itemclick']);

// 向tauri查询文件列表
let list = reactive([]);

const showContent = (target) => {
  emit('itemclick', target.filePath || "")
}

const handlelistShow = async (res) => {
  list.push(res.data);
  await setData({key: "list", val: res.data})
}

onMounted(async() => {
  try {
    let store = await getData("list");
    if(store) list.push(store);
    console.log(store, "=========get store");
    event.listen("md-list", (res) => {
      console.log(res, "======res");
      list = res && res.payload.data;
    });
  } catch(e) {
    console.log(e, "======list vue mounted");
  }
})

</script>

<template>
  <div>
    <ListItemVue
      v-for="item in list"
      :key="item"
      :title="item.title"
      :create-time="item.createTime"
      :count="item.count"
      :desc="item.desc"
      @click="showContent(item)"
    ></ListItemVue>
    <EmptyVue v-if="!list.length" @showlist="handlelistShow"/>
  </div>
</template>

<style scoped>

</style>