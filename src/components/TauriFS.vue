<script setup lang="ts">
import { ref } from "vue";
import { readBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";
import { resourceDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";

const imgSrc = ref();

const readImg = async () => {
  const contents = await readBinaryFile("img/avatar.png", {
    dir: BaseDirectory.Resource,
  });
  console.log(contents);
};

const readImgTwo = async () => {
  const resourceDirPath = await resourceDir();
  const filePath = await join(resourceDirPath, "img/avatar.png");
  const assetUrl = convertFileSrc(filePath);
  console.log(`resourceDirPath:${resourceDirPath} ,\n filePath:${filePath} \n ,assetUrl:${assetUrl}`)
  imgSrc.value = assetUrl;
};
</script>

<template>
  <button @click="readImg">读取文件指定路径下的文件(前端TS)</button>
  <button @click="readImgTwo">通过路径直接显示文件（前端TS）</button>
  <img :src="imgSrc" alt="测试图片img/avatar.png"/>
</template>
<style scoped>
</style>