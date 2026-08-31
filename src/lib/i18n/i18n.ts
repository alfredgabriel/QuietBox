import { writable, derived } from 'svelte/store';
import en from './en.json';
import es from './es.json';

export type Locale = 'es' | 'en';

const translations: Record<Locale, Record<string, string>> = { en, es };

export const currentLocale = writable<Locale>('es');

export const t = derived(currentLocale, ($locale) => {
  return (key: string): string => {
    return translations[$locale]?.[key] || translations['en']?.[key] || key;
  };
});
