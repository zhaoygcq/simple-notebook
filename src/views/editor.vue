<script setup>
import { onMounted, ref, watch, watchEffect } from "vue";
import { Editor } from '@bytemd/vue-next';
import { event } from "@tauri-apps/api";
import 'bytemd/dist/index.css';
import { debounce } from "../utils";
import { getContentApi, saveContentApi } from "../api/file"

let props = defineProps(['currentPath'])

const text = ref("");

// 保存文件内容
const handleSave = debounce(async (content) => {
  try {
    let res = await saveContentApi(props.currentPath, content);
  } catch (e) {
    console.log(e, "=====save error=====");
  }
}, 500)

const handleChange = (val) => {
  text.value = val;
  console.log("change");
  handleSave(val);
}

const getContent = async (path) => {
  let content = await getContentApi(path)

  text.value = content;
  console.log(content, "=========content=======", path);
}

onMounted(async () => {
  console.log("======editor", props.currentPath);
  try {
    await getContent(props.currentPath);
  } catch (e) {
    console.log("get content error=======", e);
  }
})

watch(() => props.currentPath, async (newContent, oldContent) => {
  console.log(newContent, "========", oldContent);

  await getContent(props.currentPath);
})
</script>

<template>
  <Editor class="editor" :value="text" @change="handleChange" />
</template>

<style>
.editor .bytemd {
  height: 100vh;
  text-align: left;
  border: 0;
}
</style>