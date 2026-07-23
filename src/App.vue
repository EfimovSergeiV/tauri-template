<script setup>
import { onMounted, ref } from "vue";
import { RouterLink, RouterView, useRoute } from "vue-router";

const isDark = ref(false);
const route = useRoute();

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
</script>

<template>
  <main class="min-h-screen bg-gray-300 px-6 py-10 text-gray-900 transition-colors dark:bg-gray-600 dark:text-gray-100">
    <div class="mx-auto flex w-full max-w-5xl flex-col gap-6">
      <header class="rounded-2xl bg-white p-5 shadow-lg shadow-gray-500/20 dark:bg-gray-800">
        <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
          <div>
            <p class="text-sm uppercase tracking-[0.2em] text-gray-500 dark:text-gray-400">Tauri + Vue Router</p>
            <h1 class="mt-1 text-2xl font-semibold">Навигация между страницами</h1>
          </div>

          <div class="flex items-center gap-3">
            <nav class="flex rounded-xl bg-gray-100 p-1 dark:bg-gray-900">
              <RouterLink
                to="/"
                class="rounded-lg px-4 py-2 text-sm transition"
                :class="route.name === 'home' ? 'bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900' : 'text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white'"
              >
                Главная
              </RouterLink>
              <RouterLink
                to="/about"
                class="rounded-lg px-4 py-2 text-sm transition"
                :class="route.name === 'about' ? 'bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900' : 'text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white'"
              >
                About
              </RouterLink>
            </nav>

            <button
              type="button"
              class="shrink-0 rounded border border-gray-400 px-3 py-2 text-sm hover:bg-gray-200 dark:border-gray-300 dark:hover:bg-gray-700"
              @click="toggleTheme"
            >
              {{ isDark ? "Светлая тема" : "Тёмная тема" }}
            </button>
          </div>
        </div>
      </header>

      <RouterView v-slot="{ Component, route: currentRoute }">
        <Transition name="page" mode="out-in">
          <component :is="Component" :key="currentRoute.fullPath" />
        </Transition>
      </RouterView>
    </div>
  </main>
</template>


<style scoped>
</style>
