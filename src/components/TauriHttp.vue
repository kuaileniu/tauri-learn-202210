<script setup lang="ts">
import { ref } from "vue";
import { fetch } from "@tauri-apps/api/http";

type Respomse = {
  data: string,
}

const tianQiData = ref("");

const fetchTianQi = async () => {
  const response: Respomse = await fetch(
    "https://v0.yiketianqi.com/api?unescape=1&version=v91&appid=43656176&appsecret=I42og6Lm&ext=&cityid=&city=",
    {
      method: "GET",
      timeout: 30,
    }
  );
  console.log(response.data);
  tianQiData.value = response.data; // 如何在 vscode 内 去除红色下划线警示？
};
</script>
<template>
  <button @click="fetchTianQi">调用Http</button>
  <p>{{ tianQiData }}</p>
</template>
<style scoped>
</style>