<script setup>
import { onMounted, reactive } from 'vue';
import ListItemVue from '../components/ListItem.vue';
import EmptyVue from "../components/Empty.vue"
import { invoke, event } from "@tauri-apps/api";
// 向tauri查询文件列表
let list = reactive([]);

const showContent = (target) => {
  console.log("查看某个文件内容");
}

const handlelistShow = (res) => {
  console.log(res, "=======res");
  list.push(res.data);
}

onMounted(async() => {
  try {
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
      :title="item.title"
      :create-time="item.createTime"
      :count="item.count"
      :desc="item.desc"
      @showContent="showContent"
    ></ListItemVue>
    <EmptyVue v-if="!list.length" :showlist="handlelistShow"/>
  </div>
</template>

<style scoped>

</style>