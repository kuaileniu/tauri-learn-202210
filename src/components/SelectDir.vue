<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { appDir } from "@tauri-apps/api/path";
const imgSrc = ref();

const selectDir = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await appDir(),
  });
  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single directory
  }
  console.log(`dir: ${selected}`)
};
</script>

<template>
  <button @click="selectDir">选取文件夹</button>
</template>
<style scoped>
</style>