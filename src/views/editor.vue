<script setup>
import { onMounted, ref, } from "vue";
import { Editor } from '@bytemd/vue-next';
import { event } from "@tauri-apps/api";
import 'bytemd/dist/index.css';
import { invoke } from "@tauri-apps/api/tauri";
import { debounce } from "../utils";

let props = defineProps(['currentPath'])

const text = ref("");

// 设置一个监听函数，用于接收tauri读取的文本内容
onMounted(() => {
  console.log("======editor", props.currentPath);
})

// 保存文件内容
const handleSave = debounce(async (content) => {
  try {
    let res = await invoke("save_content", {
      filepath: props.currentPath,
      content
    })
  }catch(e) {
    console.log(e, "=====save error=====");
  }
}, 500)

const handleChange = (val) => {
  text.value = val;
  console.log("change");
  handleSave(val); 
}

onMounted(async () => {
  try {
    let content = await invoke("get_content", {
      filepath: props.currentPath
    });

    if(content) text.value = content;
    console.log(content, "=========content=======");
  }catch(e) {
    console.log("get content error=======", e);
  }
})
</script>

<template>
  <Editor class="editor" :value="text" @change="handleChange" />
</template>

<style>
.editor .bytemd {
  height: 100vh;
  text-align: left;
  border-left: 0;
}
</style>