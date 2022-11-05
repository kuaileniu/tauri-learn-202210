<script setup lang="ts">
import { defineComponent } from "vue";
import { appWindow, WebviewWindow } from "@tauri-apps/api/window";
const openWindow_1 = async () => {
  // const mainWindow = WebviewWindow.getByLabel('main')
  // await appWindow.hide(); // 隐藏主窗口,appWindow 默认是main窗口
  const webview = new WebviewWindow("window_1", {
    alwaysOnTop: true,
    title: "新建窗口-1",
    url: "page/window_1.html",
    width: 300,
    height: 300,
  });
  
  webview.once("tauri://created", function () {
    // webview window successfully created
    console.log("window_1 窗口创建成功");
    // appWindow.close(); // 关闭主窗口
  });
  webview.once("tauri://error", function (e) {
    // an error happened creating the webview window
    console.log("window_1 窗口创建失败");
  });
};
const openWindow_2 = async () => {
  const webview = new WebviewWindow("window_2", {
    alwaysOnTop: true,
    title: "新建窗口-2",
    url: "page/window_2.html",
    x: 100,
    y: 100,
    width: 300,
    height: 300,
  });
 
  webview.once("tauri://created", function () {
    // webview window successfully created
    console.log("window_2 窗口创建成功");
    // appWindow.close(); // 关闭主窗口
  });
  webview.once("tauri://error", function (e) {
    // an error happened creating the webview window
    console.log("window_2 窗口创建失败");
  });
};
</script>
<template>
  <button @click="openWindow_1">开启窗口-1</button>
  <button @click="openWindow_2">开启窗口-2</button>
</template>
<style scoped>
</style>