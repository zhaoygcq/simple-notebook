<script setup>
import Loading from "../assets/loading.png";
defineProps(['text'])
</script>

<template>
  <div class="loading-container">
    <img class="icon" :src="Loading" />
    <p>{{text}}</p>
  </div>
</template>

<style scoped>
.loading-container {
    margin-top: 10px;
    text-align: center;
}
.icon {
    height: 24px;
    animation: myrotate 1s linear infinite;
}

@keyframes myrotate {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

p {
    margin: 0;
    padding: 0;
}
</style>