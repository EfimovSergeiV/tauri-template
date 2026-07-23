import { createI18n } from "vue-i18n";
import en from "../locales/en.json";
import ru from "../locales/ru.json";

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

const i18n = createI18n({
  legacy: false,
  locale: resolveInitialLocale(),
  fallbackLocale: "en",
  messages: {
    en,
    ru,
  },
});

export function persistLocale(locale) {
  localStorage.setItem(STORAGE_KEY, locale);
}

export default i18n;