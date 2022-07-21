<script setup>
import { onMounted, ref } from 'vue';
import editorVue from './views/editor.vue';
import listVue from './views/list.vue';
import ActivityItemVue from './components/ActivityItem.vue';
import SearchVue from "./views/Search.vue";
import { Activities } from './common/constants';
import { listenDoForFileApi } from "./api/file";

const currentPath = ref('');
const checked = ref('file');
const listContainer = ref(null);
const showList = ref(true);

const handleCurrentPath = (path) => {
  currentPath.value = path;
}

const changeActivityItem = (id) => {
  // 点击当前展示的activity项，用于控制List区域的展示与隐藏
  if(id === checked.value) {
    showList.value = !showList.value;
  } else {
    !showList.value ? showList.value = true : '';
    checked.value = id;
  }
}

const toggleListStatus = () => {
  showList.value = !showList.value;
}

// 处理文件丢失的问题
const handleFileNotFound = (data) => {
  console.log("hanlde file not found", data, listContainer.value);
  listContainer.value && listContainer.value.updateForce(data);
}

onMounted(() => {
  listenDoForFileApi({
    'hide_sidebar': toggleListStatus,
  });
})
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
  <section class="list" v-show="showList">
    <listVue
      ref="listContainer"
      v-show="checked === 'file'"
      @itemclick="handleCurrentPath"
    />
    <!-- 搜索区域 -->
    <SearchVue
      v-show="checked === 'search'"
      @itemclick="handleCurrentPath"
    />
    <!-- 提纲区域 -->
  </section>
  <section class="content">
    <editorVue
      v-if="currentPath"
      :current-path="currentPath"
      @file-not-found="handleFileNotFound"
    />
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
  /* border-top: 1px solid #e1e4e8; */
  box-sizing: border-box;
  height: 100vh;
  overflow: hidden;
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
