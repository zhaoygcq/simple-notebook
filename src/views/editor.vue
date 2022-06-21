<script setup>
import { ref } from "vue";

const text = ref("");

</script>

<template>
  <div id="md-editor-v3">
    <md-editor-v3 v-model="text" />
  </div>
</template>

<style>

</style>