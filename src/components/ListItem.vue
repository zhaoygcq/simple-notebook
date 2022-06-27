<script setup>
import { computed } from 'vue';
let props = defineProps({
    count: Number,
    createTime: Number,
    desc: String,
    title: String
})

let tranformedCreateTime = computed(() => {
    console.log(props.createTime)
    let date = new Date(props.createTime)
    return date.toLocaleDateString()+ " " + date.toLocaleTimeString();
})

let showItemDetail = () => {
    console.log("点击左侧菜单栏")
}
</script>

<template>
  <div class="list-item" @click="showItemDetail">
    <p class="title" :title="title">{{title || "Untitled"}}</p>
    <p class="desc">{{desc || ""}}</p>
    <p class="info">
        <span>字数{{count || 0}}</span>
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
    border-bottom: 1px solid #e1e4e8;
    cursor: pointer;
}
.title {
    margin: 6px;
    font-size: 14px;
    font-weight: bold;
    text-align: left;
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