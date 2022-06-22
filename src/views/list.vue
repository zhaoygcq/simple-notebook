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
      :count="item.count"
      :create-time="item.createTime"
      @showContent="showContent"
    ></ListItemVue>
    <EmptyVue v-if="!list.length" />
  </div>
</template>

<style scoped>

</style>