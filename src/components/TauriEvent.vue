<script setup lang="ts">
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

const initProcess = async () => {
  await invoke("init_process");
  console.log("通知rust端(只通知一次)-------------------");
  await listen<string>("event-name-hggfjt", (event) => {
    console.log(event.payload); // event.payload 对应rust端端对象;TODO 专成TS的类？
    console.log(event);
  });
};
</script>
<template>
  <button @click="initProcess">触发事件(rust给ts推送对象信息)</button>
</template>
<style scoped>
</style>
