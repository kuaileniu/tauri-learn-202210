<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";

const minWindow = () => {
  appWindow.minimize();
};
const maxWindow = () => {
  appWindow.toggleMaximize();
};
const closeWindow = () => {
  appWindow.close();
};
</script>
<template>
 
    <div data-tauri-drag-region class="titlebar">
      <div @click="minWindow" class="titlebar-button" id="titlebar-minimize">
        <img  
         src="/mdi_window-minimize.svg"
          alt="minimize"
        />
      </div>
      <div @click="maxWindow" class="titlebar-button" id="titlebar-maximize">
          <img src="/mdi_window-maximize.svg"
          alt="maximize"
        />
      </div>
      <div @click="closeWindow" class="titlebar-button" id="titlebar-close">
        <img src="/mdi_close.svg" 
        alt="close" />
      </div>
    </div>
 
</template>
<style scoped>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
 
</style>