<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const isDark = ref(false);

function applyTheme(value) {
  document.documentElement.classList.toggle("dark", value);
}

function toggleTheme() {
  isDark.value = !isDark.value;
  applyTheme(isDark.value);
  localStorage.setItem("theme", isDark.value ? "dark" : "light");
}

onMounted(() => {
  const savedTheme = localStorage.getItem("theme");
  if (savedTheme === "dark" || savedTheme === "light") {
    isDark.value = savedTheme === "dark";
  } else {
    isDark.value = window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
  applyTheme(isDark.value);
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="container bg-gray-300 text-gray-900 transition-colors dark:bg-gray-600 dark:text-gray-100">
    <button
      type="button"
      class="mb-4 rounded border border-gray-400 px-3 py-1 text-sm hover:bg-gray-200 dark:border-gray-300 dark:hover:bg-gray-700"
      @click="toggleTheme"
    >
      {{ isDark ? "Светлая тема" : "Тёмная тема" }}
    </button>

    <h1 class="text-red-500 dark:text-blue-500">Привет мир</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
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

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>


<style scoped>
</style>
