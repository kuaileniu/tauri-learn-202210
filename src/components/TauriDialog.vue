<script setup languge="ts">
import { ref } from "vue";
import { ask, confirm, open } from "@tauri-apps/api/dialog";

const imgPath = ref();
const askDialogOne = async () => {
  //   const yes = await ask("你确认吗Are you sure?", "确认提示Tauri");
  const yes = await ask("你确认吗Are you sure?");
  console.log(typeof yes);
  console.log(yes);
};

const askDialogTwo = async () => {
  const yes2 = await ask("This action cannot be reverted. Are you sure?", {
    title: "标题Tauri",
    type: "warning",
  });
  console.log(typeof yes2);
  console.log(yes2);
};

const confirmDialogOne = async () => {
  const confirmed = await confirm("您确认吗Are you sure?", "Tauri");
  console.log(confirmed);
};
const confirmDialogTwo = async () => {
  const confirmed = await confirm(
    "This action cannot be reverted. Are you sure?",
    { title: "Tauri", type: "warning" }
  );
  console.log(confirmed);
};

const openDialogOne = async () => {
  // Open a selection dialog for image files
  const selected = await open({
    // directory:true,
    multiple: true,
    filters: [
      {
        name: "Image",
        // extensions: ["png", "jpeg"],
        extensions: ["PNG", "JPG"], // 区分大小写
        // extensions: ["*"],
      },
    ],
  });
  console.log(selected); //["/Volumes/HD/img/101CANON/IMG_0041.JPG"]

  if (Array.isArray(selected)) {
    imgPath.value = selected[0];
    // user selected multiple files
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single file
    imgPath.value = selected;
  }
  console.log(`imgPath : ${imgPath.value}`);
};
</script>
<template>
  <button @click="askDialogOne">弹出框1</button>
  <button @click="askDialogTwo">弹出框2</button>
  <button @click="confirmDialogOne">确认框1</button>
  <button @click="confirmDialogTwo">确认框2</button>
  <button @click="openDialogOne">打开文件选择窗口</button>
</template>
<style scoped>
</style>