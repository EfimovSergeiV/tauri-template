<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const inputValue = ref("");
const result = ref("");
const errorMessage = ref("");
const isLoading = ref(false);

async function multiplyByFour() {
  errorMessage.value = "";
  result.value = "";

  const numericValue = Number(inputValue.value);
  if (Number.isNaN(numericValue)) {
    errorMessage.value = "Введите корректное число.";
    return;
  }

  isLoading.value = true;

  try {
    const response = await invoke("multiply_by_four", { value: numericValue });
    result.value = String(response);
  } catch (error) {
    errorMessage.value = typeof error === "string" ? error : "Не удалось выполнить Python-скрипт.";
  } finally {
    isLoading.value = false;
  }
}
</script>


<template>
  <section class="w-full max-w-md rounded-2xl bg-white p-6 shadow-lg shadow-gray-500/20 dark:bg-gray-800">
    <div class="mb-6">
      <h2 class="text-2xl font-semibold">Python в Tauri</h2>
      <p class="mt-1 text-sm text-gray-600 dark:text-gray-300">
        Введите число, приложение передаст его в Python и покажет результат умножения на 4.
      </p>
    </div>

    <label class="mb-2 block text-sm font-medium" for="number-input">
      Число
    </label>
    <input
      id="number-input"
      v-model="inputValue"
      type="number"
      step="any"
      class="w-full rounded-lg border border-gray-300 bg-gray-50 px-4 py-3 outline-none transition focus:border-gray-500 dark:border-gray-600 dark:bg-gray-900"
      placeholder="Например, 3.5"
      @keyup.enter="multiplyByFour"
    />

    <button
      type="button"
      class="mt-4 w-full rounded-lg bg-gray-900 px-4 py-3 font-medium text-white transition hover:bg-gray-700 disabled:cursor-not-allowed disabled:opacity-60 dark:bg-gray-100 dark:text-gray-900 dark:hover:bg-gray-300"
      :disabled="isLoading"
      @click="multiplyByFour"
    >
      {{ isLoading ? "Вычисляем..." : "Умножить на 4" }}
    </button>

    <div class="mt-4 h-10">
      <p v-if="result" class="rounded-lg bg-green-100 px-4 py-3 text-green-900 dark:bg-green-900/40 dark:text-green-100">
        Результат: {{ result }}
      </p>
    </div>

    <p v-if="errorMessage" class="mt-4 rounded-lg bg-red-100 px-4 py-3 text-red-900 dark:bg-red-900/40 dark:text-red-100">
      {{ errorMessage }}
    </p>
  </section>
</template>