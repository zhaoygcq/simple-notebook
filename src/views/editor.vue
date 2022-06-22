<script setup>
import { onMounted, ref, } from "vue";
import { Editor } from '@bytemd/vue-next'
import { event } from "@tauri-apps/api";
import 'bytemd/dist/index.css'

const text = ref("");
const handleChange = (val) => {
  text.value = val;
  console.log("change")
}

// 设置一个监听函数，用于接收tauri读取的文本内容
onMounted(() => {
  event.listen("show-md-content", (res) => {
    console.log(res, "======res");
    text.value = res && res.payload.data;
  });
})

</script>

<template>
  <Editor class="editor" :value="text" @change="handleChange"/>
</template>

<style>
.editor .bytemd {
  height: 100vh;
  text-align: left;
}
</style>