<script setup lang="ts">
import { ref } from "vue";
import { readBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";
import { resourceDir, join, resolveResource } from "@tauri-apps/api/path";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";

const imgSrc = ref();
const imgBinaryFile = ref();
const imgMemory = ref();

const readImgBinaryFile = async () => {
  const contents = await readBinaryFile("img/avatar.png", {
    dir: BaseDirectory.Resource,
  });
  console.log(contents);
  imgBinaryFile.value ="blob:" + contents
  // imgBinaryFile.value ="data:image/png;base64," + contents
};

const readImgFromDisk = async () => {
  const resourceDirPath = await resourceDir();
  const filePath = await join(resourceDirPath, "img", "avatar.png");
  const assetUrl = await convertFileSrc(filePath, "asset://localhost");
  console.log(
    `resourceDirPath:${resourceDirPath} ,\n filePath:${filePath} \n ,assetUrl:${assetUrl}`
  );
  imgSrc.value = assetUrl;
};

const readImgFromMemory = async () => {
  const resourceDirPath = await resourceDir();
  const filePath = await join(resourceDirPath, "img", "avatar.png");
  const content = await invoke("read_img_file", {
    path: filePath,
  });
};

</script>

<template>
 
  <button @click="readImgFromMemory">显示文件内存文件</button>
  <img :src="imgMemory" alt="image form memory"/>
  <button @click="readImgFromDisk">前端TS从app内dist读取图片</button>
  <img :src="imgSrc" alt="测试图片img/avatar.png" />
  <img
    src="/avatar.png"
    class="logo vite"
    alt="public/avatar.png;public下的文件可被直接引用"
  />
</template>
<style scoped>
</style>
