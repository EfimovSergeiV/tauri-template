import { createApp } from "vue";
import { createPinia } from "pinia";
import { createI18n } from "vue-i18n";
import router from "./router";

import en from "./locales/en.json";
import ru from "./locales/ru.json";

import "./style.css";
import App from "./App.vue";

const STORAGE_KEY = "locale";
const availableLocales = ["ru", "en"];

function resolveInitialLocale() {
  const savedLocale = localStorage.getItem(STORAGE_KEY);
  if (savedLocale && availableLocales.includes(savedLocale)) {
    return savedLocale;
  }

  const browserLocale = navigator.language.split("-")[0];
  return availableLocales.includes(browserLocale) ? browserLocale : "ru";
}

const pinia = createPinia();


const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: resolveInitialLocale(),
  fallbackLocale: "en",
  messages: {
    en,
    ru,
  },
});


const app = createApp(App);
app.use(router);
app.use(pinia);
app.use(i18n);
app.mount("#app");
