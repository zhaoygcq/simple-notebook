<script setup>
import { ref, reactive, onMounted } from 'vue';
import SearchResItemVue from '../components/SearchResItem.vue';
import LoadingVue from '../components/Loading.vue';
import { getSearchRes } from "../utils";
import { getData } from "../store/store";
import { debounce } from '../utils';

const emit = defineEmits(['itemClick']);

const searchVal = ref("");
// 控制loading状态
const loadingStatus = ref(false);
// 控制搜索结果的展示
const showSearchRes = ref(false);
const searchRes = reactive({
    list: []
});

const handleSearchValChange = debounce(async () => {
    searchRes.list = [];
    if(!searchVal.value) return;
    showSearchRes.value = true;
    loadingStatus.value = true;
    console.log(searchVal.value, "======searchVal change");
    // 进行内容搜索
    let allFiles = await getData("list");
    let res = await getSearchRes(searchVal.value, allFiles);
    console.log(res, "======search res");
    loadingStatus.value = false;
    searchRes.list = res;
}, 500)


// 搜索内容点击事件
const handleSearchItemClick = (item) => {
    console.log("=======search item click", item);
    emit('itemclick', item.originPath);
}
</script>

<template>
  <div class="search-container">
    <input
      autofocus
      placeholder="输入关键字查询"
      v-model="searchVal"
      @input="handleSearchValChange"
    />
    <div class="search-contents" v-if="showSearchRes && searchVal.length">
        <div v-if="!loadingStatus" class="contents">
            <SearchResItemVue
                v-for="(item, index) in searchRes.list"
                :key="index"
                :desc="item.desc"
                :origin="item.origin"
                :origin-desc="item.originDesc"
                @click="handleSearchItemClick(item)"
            />
            <p class="no-res" v-if="!searchRes.list.length">暂未搜索到结果</p>
        </div>
        <LoadingVue v-else text="Loading..." />
    </div>

  </div>
</template>

<style scoped>
.search-container {
    display: flex;
    flex-direction: column;
}

input {
    margin: 20px 10px;
    height: 20px;
}

.search-contents,
.contents {
    display: flex;
    flex-direction: column;
    border-top: 1px solid #e1e4e8;
}

.contents {
    border: 0;
}

.no-res {
    text-align: center;
    font-size: 12px;
}
</style>