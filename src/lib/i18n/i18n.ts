import { writable, derived } from 'svelte/store';
import en from './en.json';
import es from './es.json';
import fr from './fr.json';
import de from './de.json';

export type Locale = 'en' | 'es' | 'fr' | 'de';

const translations: Record<Locale, Record<string, string>> = { en, es, fr, de };

export const currentLocale = writable<Locale>('en');

export const t = derived(currentLocale, ($locale) => {
  return (key: string): string => {
    return translations[$locale]?.[key] || translations['en']?.[key] || key;
  };
});
