<script setup >
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Greet from "./components/Greet.vue";
import InOut from "./components/InOut.vue";
import TauriEvent from "./components/TauriEvent.vue";
import TauriHttp from "./components/TauriHttp.vue";
import TauriFS from "./components/TauriFS.vue";
import TauriDialog from "./components/TauriDialog.vue";
import TauriFSEvery from "./components/TauriFSEvery.vue";
import WindowCustomization from "./components/WindowCustomization.vue";
import SelectDir from "./components/SelectDir.vue";
import Multiwindow from "./components/Multiwindow.vue";
import RunElf from "./components/RunElf.vue";

onMounted(() => {
  // setTimeout(() => {
  //   console.log("前端关闭开屏界面");
  //   invoke("close_splashscreen");
  // }, 3000);
});
</script>

<template>
  <div class="btn-group">
    <button>最上面的按钮</button>
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
          <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

      <p>
        Recommended IDE setup:
        <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>
        +
        <a href="https://github.com/johnsoncodehk/volar" target="_blank"
          >Volar</a
        >
        +
        <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"
          >Tauri</a
        >
        +
        <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"
          >rust-analyzer</a
        >
      </p>

      <Greet />
      <InOut />
      <TauriEvent />
      <TauriHttp />
      <TauriFS />
      <TauriDialog />
      <SelectDir />
      <TauriFSEvery />
      <WindowCustomization />
      <Multiwindow />
      <RunElf />
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
.btn-group {
  margin-top: 35px;
}
</style>
