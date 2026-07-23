<script setup>
  import { onBeforeUnmount, ref } from "vue";
  import { useI18n } from "vue-i18n";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { useCounterStore } from "../stores/counter";
  import ThreeJS from "../components/graphics/ThreeJS.vue";

  const counter = useCounterStore();

  const inputValue = ref("");
  const result = ref("");
  const errorMessage = ref("");
  const isLoading = ref(false);
  const isCounting = ref(false);
  const counterValues = ref([]);
  const counterError = ref("");
  const counterStatus = ref("");
  const { t } = useI18n();

  let unlistenCounterValue = null;
  let unlistenCounterDone = null;
  let unlistenCounterError = null;
  let unlistenCounterStopped = null;

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

  function clearCounterListeners() {
    if (typeof unlistenCounterValue === "function") {
      unlistenCounterValue();
      unlistenCounterValue = null;
    }

    if (typeof unlistenCounterDone === "function") {
      unlistenCounterDone();
      unlistenCounterDone = null;
    }

    if (typeof unlistenCounterError === "function") {
      unlistenCounterError();
      unlistenCounterError = null;
    }

    if (typeof unlistenCounterStopped === "function") {
      unlistenCounterStopped();
      unlistenCounterStopped = null;
    }
  }

  async function runCounterScript() {
    if (isCounting.value) return;

    counterError.value = "";
    counterStatus.value = "";
    counterValues.value = [];

    clearCounterListeners();

    unlistenCounterValue = await listen("counter-stream-value", (event) => {
      const number = Number(event.payload);
      if (Number.isFinite(number)) {
        counterValues.value.push(number);
      }
    });

    unlistenCounterDone = await listen("counter-stream-done", () => {
      counterStatus.value = "Скрипт завершён";
      isCounting.value = false;
      clearCounterListeners();
    });

    unlistenCounterError = await listen("counter-stream-error", (event) => {
      counterError.value = String(event.payload ?? "Не удалось запустить скрипт счётчика");
      isCounting.value = false;
      clearCounterListeners();
    });

    unlistenCounterStopped = await listen("counter-stream-stopped", () => {
      counterStatus.value = "Скрипт остановлен";
      isCounting.value = false;
      clearCounterListeners();
    });

    isCounting.value = true;

    try {
      await invoke("count_one_to_hundred_stream");
    } catch (error) {
      counterError.value = typeof error === "string" ? error : "Не удалось запустить скрипт счётчика";
      isCounting.value = false;
      clearCounterListeners();
    }
  }

  async function stopCounterScript() {
    try {
      const stopped = await invoke("stop_count_one_to_hundred_stream");
      if (!stopped) {
        counterStatus.value = "Скрипт не запущен";
      }
    } catch (error) {
      counterError.value = typeof error === "string" ? error : "Не удалось остановить скрипт";
    }
  }

  onBeforeUnmount(() => {
    clearCounterListeners();
  });
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

      <button
        type="button"
        class="mt-4 w-full rounded-lg bg-blue-700 px-4 py-3 font-medium text-white transition hover:bg-blue-600 disabled:cursor-not-allowed disabled:opacity-60"
        :disabled="isCounting"
        @click="runCounterScript"
      >
        {{ isCounting ? "Скрипт выполняется..." : "Запустить скрипт 1-100" }}
      </button>

      <button
        type="button"
        class="mt-3 w-full rounded-lg bg-red-700 px-4 py-3 font-medium text-white transition hover:bg-red-600 disabled:cursor-not-allowed disabled:opacity-60"
        :disabled="!isCounting"
        @click="stopCounterScript"
      >
        Стоп
      </button>

      <div class="mt-3 h-10">
        <p
          v-if="counterValues.length"
          class="rounded-lg bg-blue-100 px-4 py-3 text-blue-900 dark:bg-blue-900/40 dark:text-blue-100"
        >
          Получено {{ counterValues.length }} чисел. Последнее: {{ counterValues[counterValues.length - 1] }}
        </p>
      </div>

      <p v-if="counterStatus" class="mt-3 rounded-lg bg-gray-100 px-4 py-3 text-gray-900 dark:bg-gray-700 dark:text-gray-100">
        {{ counterStatus }}
      </p>

      <p v-if="counterError" class="mt-4 rounded-lg bg-red-100 px-4 py-3 text-red-900 dark:bg-red-900/40 dark:text-red-100">
        {{ counterError }}
      </p>
    </section>

    <section class="flex">


      <p class="font-semibold">ЧИСЛО ИЗ STORE: {{ counter.valueKw }}</p>

      <ThreeJS />

      <img src="../assets/mask.png" class="h-24" alt="Vite logo" />

    </section>
  </div>

</template>