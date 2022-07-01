<script setup>
defineProps([
    'icon',
    'checkedIcon',
    'checked'
])
</script>

<template>
  <div class="icon-container">
    <img :src="icon" v-if="!checked" />
    <img :src="checkedIcon" v-else />
  </div>
</template>

<style scoped>
.icon-container {
    padding: 10px;
    cursor: pointer;
}

img {
    width: 24px;
    height: 24px;
}
</style>