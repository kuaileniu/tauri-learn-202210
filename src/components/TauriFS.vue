<script setup lang="ts">
import { readBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";
// Copy the `$APPDIR/app.conf` file to `$APPDIR/app.conf.bk`
// await copyFile('app.conf', 'app.conf.bk', { dir: BaseDirectory.App });

const readImg = async () => {
  // Read the image file in the `$RESOURCEDIR/avatar.png` path
  const contents = await readBinaryFile("img/avatar.png", {
    dir: BaseDirectory.Resource,
  });
  console.log(contents)
};
</script>

<template>
    <button @click="readImg">读取文件指定路径下的文件(前端TS)</button>
</template>
<style scoped>

</style>