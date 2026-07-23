<script setup>
import { useI18n } from "vue-i18n";

const { t } = useI18n();
</script>

<template>
  <section class="w-full max-w-2xl rounded-2xl bg-white p-6 shadow-lg shadow-gray-500/20 dark:bg-gray-800">
    <h2 class="text-2xl font-semibold">{{ t("pages.about.title") }}</h2>
    <p class="mt-3 text-gray-700 dark:text-gray-300">
      {{ t("pages.about.description1") }}
    </p>
    <p class="mt-3 text-gray-700 dark:text-gray-300">
      {{ t("pages.about.description2") }}
    </p>
    <p class="mt-3 text-gray-700 dark:text-gray-300">
      {{ t("pages.about.description3") }}
    </p>
  </section>
</template>