<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { resourceDir, join, resolveResource } from "@tauri-apps/api/path";
const elfMsg = ref("");
 

async function run4Elf() {
  console.log("elf.......")
  const resourceDirPath = await resourceDir();
  const filePath = await join(resourceDirPath, "elf/macos-x64", "elf");
  elfMsg.value = await invoke("run_elf", {
    path: filePath,
  });
}

</script>

<template>
  <div class="card">
    <button type="button" @click="run4Elf()">Run ELF 文件</button>
    <span>{{ elfMsg }}</span>
  </div>

  <p>{{ greetMsg }}</p>
</template>
