const STORAGE_KEY = "locale";

export function persistLocale(locale) {
  localStorage.setItem(STORAGE_KEY, locale);
}
