<script setup>
import { onMounted } from "vue";
import file from "../assets/fileChecked.png"
defineProps({
    origin: String,
    originDesc: String,
    desc: Array
})
</script>

<template>
  <div class="search-res-item">
    <p class="title">
        <img :src="file" class="icon"/>
        <span class="origin">{{origin}}</span>
        <span class="origin-desc">{{originDesc}}</span>
    </p>
    <p v-for="item in desc">{{item}}</p>
  </div>
</template>

<style scoped>
p {
    margin: 0;
    padding: 0;
    font-size: 12px;
}
.search-res-item {
    margin: 10px 0;
    padding-left: 10px;
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid #e1e4e8;
    cursor: pointer;
}

.title {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.icon {
    width: 12px;
    vertical-align: baseline;
}

.origin {
    color: lightblue;
}

.origin-desc {
    margin-left: 10px;
    font-size: 10px;
}
</style>