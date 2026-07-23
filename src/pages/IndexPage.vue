<script setup>
  import { ref } from "vue";
  import { useI18n } from "vue-i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { useCounterStore } from '../stores/counter'
import ThreeJS from "../components/graphics/ThreeJS.vue";
  
  const counter = useCounterStore()

  const inputValue = ref("");
  const result = ref("");
  const errorMessage = ref("");
  const isLoading = ref(false);
  const { t } = useI18n();

  async function multiplyByFour() {
    errorMessage.value = "";
    result.value = "";

    const numericValue = Number(inputValue.value);
    if (Number.isNaN(numericValue)) {
      errorMessage.value = t("pages.home.errors.invalidNumber");
      return;
    }

    isLoading.value = true;

    try {
      const response = await invoke("multiply_by_four", { value: numericValue });
      result.value = String(response);
    } catch (error) {
      errorMessage.value = typeof error === "string" ? error : t("pages.home.errors.pythonFailed");
    } finally {
      isLoading.value = false;
    }
  }
</script>


<template>
  <div class="grid grid-cols-2">
    <section class="w-full max-w-md rounded-2xl bg-white p-6 shadow-lg shadow-gray-500/20 dark:bg-gray-800">
      <div class="mb-6">
        <h2 class="text-2xl font-semibold">{{ t("pages.home.title") }}</h2>
        <p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
          {{ t("pages.home.description") }}
        </p>
      </div>

      <label class="mb-2 block text-sm font-medium" for="number-input">
        {{ t("pages.home.inputLabel") }}
      </label>
      <input
        id="number-input"
        v-model="inputValue"
        type="number"
        step="any"
        class="w-full rounded-lg border border-gray-300 bg-gray-50 px-4 py-3 outline-none transition focus:border-gray-500 dark:border-gray-600 dark:bg-gray-900"
        :placeholder="t('pages.home.placeholder')"
        @keyup.enter="multiplyByFour"
      />

      <button
        type="button"
        class="mt-4 w-full rounded-lg bg-gray-900 px-4 py-3 font-medium text-white transition hover:bg-gray-700 disabled:cursor-not-allowed disabled:opacity-60 dark:bg-gray-100 dark:text-gray-900 dark:hover:bg-gray-300"
        :disabled="isLoading"
        @click="multiplyByFour"
      >
        {{ isLoading ? t("pages.home.loading") : t("pages.home.submit") }}
      </button>

      <div class="mt-4 h-10">
        <p v-if="result" class="rounded-lg bg-green-100 px-4 py-3 text-green-900 dark:bg-green-900/40 dark:text-green-100">
          {{ t("pages.home.result", { value: result }) }}
        </p>
      </div>

      <p v-if="errorMessage" class="mt-4 rounded-lg bg-red-100 px-4 py-3 text-red-900 dark:bg-red-900/40 dark:text-red-100">
        {{ errorMessage }}
      </p>
    </section>

    <section class="flex">


      <p class="font-semibold">ЧИСЛО ИЗ STORE: {{ counter.valueKw }}</p>

      <ThreeJS />

      <img src="../assets/mask.png" class="h-24" alt="Vite logo" />

    </section>
  </div>

</template>