<script setup>
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { RouterLink, RouterView, useRoute } from "vue-router";
import { persistLocale } from "./utils/locale";
import Header from "./components/Header.vue";
import Footer from "./components/Footer.vue";

const isDark = ref(false);
const route = useRoute();
const { locale, t } = useI18n();
const currentLocale = computed({
  get: () => locale.value,
  set: (value) => {
    locale.value = value;
    persistLocale(value);
  },
});

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


      <Header />

      <RouterView v-slot="{ Component, route: currentRoute }">
        <Transition name="page" mode="out-in">
          <component :is="Component" :key="currentRoute.fullPath" />
        </Transition>
      </RouterView>

      <Footer />

    </div>
  </main>
</template>


<style scoped>
</style>
