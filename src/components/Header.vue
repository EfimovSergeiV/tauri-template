<script setup>
  import { computed, onMounted, ref } from "vue";
  import { useI18n } from "vue-i18n";
  import { RouterLink, RouterView, useRoute } from "vue-router";
  import { persistLocale } from "../i18n";


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
  <header class="rounded-2xl bg-white p-5 shadow-lg shadow-gray-500/20 dark:bg-gray-800">
    <div class="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
      <div>
        <p class="text-sm uppercase tracking-[0.2em] text-gray-500 dark:text-gray-400">Tauri + Vue Router</p>
        <h1 class="mt-1 text-2xl font-semibold">{{ t("app.title") }}</h1>
      </div>

      <div class="flex items-center gap-3">
        <nav class="flex rounded-xl bg-gray-100 p-1 dark:bg-gray-900">
          <RouterLink
            to="/"
            class="rounded-lg px-4 py-2 text-sm transition"
            :class="route.name === 'home' ? 'bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900' : 'text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white'"
          >
            {{ t("menu.home") }}
          </RouterLink>
          <RouterLink
            to="/charts"
            class="rounded-lg px-4 py-2 text-sm transition"
            :class="route.name === 'home' ? 'bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900' : 'text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white'"
          >
            Charts
          </RouterLink>
          <RouterLink
            to="/about"
            class="rounded-lg px-4 py-2 text-sm transition"
            :class="route.name === 'about' ? 'bg-gray-900 text-white dark:bg-gray-100 dark:text-gray-900' : 'text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white'"
          >
            {{ t("menu.about") }}
          </RouterLink>
        </nav>

        <label class="flex items-center gap-2 rounded border border-gray-300 bg-white px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-900">
          <span class="text-gray-600 dark:text-gray-300">{{ t("menu.language") }}</span>
          <select
            v-model="currentLocale"
            class="bg-transparent outline-none"
          >
            <option value="ru">RU</option>
            <option value="en">EN</option>
          </select>
        </label>

        <button
          type="button"
          class="shrink-0 rounded border border-gray-400 px-3 py-2 text-sm hover:bg-gray-200 dark:border-gray-300 dark:hover:bg-gray-700"
          @click="toggleTheme"
        >
          {{ isDark ? t("menu.themeLight") : t("menu.themeDark") }}
        </button>
      </div>
    </div>
  </header>
</template>