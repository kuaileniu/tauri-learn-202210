<script setup lang="ts">
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

type EventPaload = {
  message: string;
  ptype: number;
};

const initProcess = async () => {
  await invoke("init_process");
  console.log("通知rust端(只通知一次)-------------------");
   // 自动转成TS的类型
  await listen<EventPaload>("event-name-hggfjt", (event) => { 
    const pay = event.payload;
    console.log(pay);
    console.log(event);
  });
};

//  独立执行
//  await listen<EventPaload>("event-name-hggfjt", (event) => { 
//    const pay = event.payload;
//    console.log(pay);
//    console.log(event);
//  });
</script>
<template>
  <button @click="initProcess">触发事件(rust给ts推送对象信息)</button>
</template>
<style scoped>
</style>
