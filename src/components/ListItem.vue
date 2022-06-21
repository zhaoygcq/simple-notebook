<script setup>
import { computed } from 'vue';
let props = defineProps({
    count: Number,
    createTime: Number
})

let tranformedCreateTime = computed(() => {
    let date = new Date(props.createTime)
    return date.toLocaleDateString()+ " " + date.toLocaleTimeString();
})

let showItemDetail = () => {
    console.log("点击左侧菜单栏")
}
</script>

<template>
  <div class="list-item" @click="showItemDetail">
    <p class="title" title="第一篇测试文章第一篇测试文章第一篇测试文章第一篇测试文章">第一篇测试文章第一篇测试文章第一篇测试文章第一篇测试文章</p>
    <p class="desc">这是一篇测试的文章，这是一篇测试的文章，这是一篇测试的文章，这是一篇测试的文章，这是一篇测试的文章，</p>
    <p class="info">
        <span>字数{{count}}</span>
        <span>修改时间:{{tranformedCreateTime}}</span>
    </p>
  </div>
</template>

<style scoped>
@import "../common/common.css";
.list-item {
    display: flex;
    flex-direction: column;
    font-size: 12px;
    padding: 10px 4px;
    border-bottom: 1px solid #ccc;
    cursor: pointer;
}
.title {
    margin: 6px;
    font-size: 14px;
    font-weight: bold;
    text-align: center;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
}

.desc {
    display: -webkit-box;
    overflow: hidden;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    text-indent: 1em;
}

.info {
    margin-top: 6px;
    display: flex;
    justify-content: space-between;
    font-size: 10px;
}
</style>