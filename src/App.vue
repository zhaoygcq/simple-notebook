<script setup>
import { ref } from 'vue';
import editorVue from './views/editor.vue';
import listVue from './views/list.vue';

const currentPath = ref('');

const handleCurrentPath = (path) => {
  currentPath.value = path;
  console.log(path, "=====currentPath=======");
}
</script>

<template>
  <section class="list">
    <listVue @itemclick="handleCurrentPath" />
  </section>
  <section class="content">
    <editorVue v-if="currentPath" :current-path="currentPath"/>
    <div v-else></div>
  </section>
</template>

<style>
body {
  margin: 0;
  padding: 0;
}
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  margin: 0;
  display: flex;
  min-height: 100vh;
}

.list {
  width: 200px;
}

.content {
  flex: 1;
}
</style>
