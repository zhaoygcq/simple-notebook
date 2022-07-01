<script setup>
import { ref } from 'vue';
import editorVue from './views/editor.vue';
import listVue from './views/list.vue';
import ActivityItemVue from './components/ActivityItem.vue';
import { Activities } from './common/constants';

const currentPath = ref('');
const checked = ref('file');
const showList = ref(true);

const handleCurrentPath = (path) => {
  currentPath.value = path;
  console.log(path, "=====currentPath=======");
}

const changeActivityItem = (id) => {
  // 点击当前展示的activity项，用于控制List区域的展示与隐藏
  if(id === checked.value) {
    showList.value = !showList.value;
  } else {
    checked.value = id;
  }
}
</script>

<template>
  <section class="activities">
    <ActivityItemVue
      v-for="item in Activities"
      :key="item.id"
      :checked="item.id === checked"
      :icon="item.icon"
      :checked-icon="item.checkedIcon"
      @click="changeActivityItem(item.id)"
    />
  </section>
  <section class="list" v-if="showList">
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

.activities {
  width: 48px;
  background-color: #555;
}

.list {
  width: 200px;
}

.content {
  flex: 1;
  border-left: 1px solid #e1e4e8;
}
</style>
