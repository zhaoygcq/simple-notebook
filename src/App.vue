<script setup>
import editorVue from './views/editor.vue';
import listVue from './views/list.vue';
</script>

<template>
  <section class="list">
    <listVue/>
  </section>
  <section class="content">
    <editorVue/>
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
  text-align: center;
  color: #2c3e50;
  margin: 0;
  display: flex;
  min-height: 100vh;
}

.list {
  width: 200px;
  border-right: 2px solid #ccc;
}

.content {
  flex: 1;
}
</style>
