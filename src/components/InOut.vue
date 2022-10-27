<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
defineProps<{ msg: String }>();
const count = ref(0);
const commandMsg = ref("");
onMounted(() => {
  const a = 0.07;
  const b = 100;
  let c = a * b;
  console.log("ts计算结果：" + c); //ts计算结果：7.000000000000001
});

// 需 args_command 的驼峰方式命名
const argsCommand = async () => {
  const a = 0.07;
  const b = 100;
  //   let c = await invoke("args_command", { a: a, b: b });
  let c = await invoke("args_command", { a, b });
  console.log("Rust 返回的值为: " + c); // Rust 计算结果为：7
  console.log(`Rust 返回的值为: ${c}`); // Rust 计算结果为：7
  commandMsg.value = c;
};
</script>
<template>
  <button @click="argsCommand">有进参和回参数方式调用Rust</button>

  <p>Rust 计算结果为: {{ commandMsg }}</p>
</template>
<style scoped>
</style>