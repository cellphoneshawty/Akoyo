import en, { type MessageKey } from './locales/en';
import ru from './locales/ru';

export type Lang = 'en' | 'ru';

export const languages: { code: Lang; name: string }[] = [
  { code: 'en', name: 'English' },
  { code: 'ru', name: 'Русский' },
];

const dictionaries: Record<Lang, Record<MessageKey, string>> = { en, ru };
const STORAGE_KEY = 'akoyo.lang';

function isLang(value: unknown): value is Lang {
  return value === 'en' || value === 'ru';
}

function detect(): Lang {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (isLang(saved)) return saved;
  } catch {
    // storage unavailable (private mode) — fall back to the browser language
  }
  return navigator.language.toLowerCase().startsWith('ru') ? 'ru' : 'en';
}

const initial = detect();
let current = $state<Lang>(initial);
document.documentElement.lang = initial;

export const locale = {
  get lang() {
    return current;
  },
  set(lang: string) {
    if (!isLang(lang)) return;
    current = lang;
    document.documentElement.lang = lang;
    try {
      localStorage.setItem(STORAGE_KEY, lang);
    } catch {
      // not critical
    }
  },
};

export function t(key: MessageKey, params?: Record<string, string | number>): string {
  let text = dictionaries[current][key] ?? en[key] ?? key;
  if (params) {
    for (const [name, value] of Object.entries(params)) {
      text = text.replaceAll(`{${name}}`, String(value));
    }
  }
  return text;
}

/** Translates an API error code, falling back to a generic message. */
export function errorText(code: string): string {
  const key = `error.${code}`;
  return key in en ? t(key as MessageKey) : t('error.internal');
}
