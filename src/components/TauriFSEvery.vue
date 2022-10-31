<script setup lang="ts">
import { ref } from "vue";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";

const filePath = ref();
const fileContent = ref("");
const readEveryTextFile = async () => {
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "text",
        extensions: ["vue", "toml"], // 区分大小写
      },
    ],
  });
  if (Array.isArray(selected)) {
    filePath.value = selected[0];
    //   // user selected multiple files
  } else if (selected === null) {
    //   // user cancelled the selection
  } else {
    //   // user selected a single file
    filePath.value = selected;
  }
  const content: string = await invoke("read_every_text_file", {
    path: filePath.value,
  });
  console.log(content);
  fileContent.value = content;
};
</script>

<template>
  <button @click="readEveryTextFile">读取系统任意文本文件(调用Rust接口)</button>
  <textarea rows="10" v-model="fileContent"></textarea>
</template>
<style scoped>
</style>