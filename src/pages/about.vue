<template>
  <section class="w-full max-w-2xl rounded-2xl bg-white p-6 shadow-lg shadow-gray-500/20 dark:bg-gray-800">
    <h2 class="text-2xl font-semibold">О проекте</h2>
    <p class="mt-3 text-gray-700 dark:text-gray-300">
      Это шаблон desktop-приложения на Tauri с фронтендом на Vue 3 и маршрутизацией через vue-router.
    </p>
    <p class="mt-3 text-gray-700 dark:text-gray-300">
      Главная страница показывает рабочий пример вызова команды Tauri, где Rust запускает Python-скрипт и возвращает результат обратно в интерфейс.
    </p>
    <p class="mt-3 text-gray-700 dark:text-gray-300">
      Страница about нужна как отдельный route, чтобы приложение уже было готово к дальнейшему разнесению экранов по страницам.
    </p>
  </section>
</template>