import { writable } from 'svelte/store';

export interface VaultFileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
}

export interface RecentArchive {
  path: string;
  name: string;
  size_bytes: number;
  opened_at: number;
  has_alt_key: boolean;
}

export type AppView = 'home' | 'create' | 'open';

export interface OpenSession {
  path: string;
  name: string;
  password: string;
  is_alt: boolean;
  has_alt_key: boolean;
  entries: VaultFileEntry[];
}

export const view = writable<AppView>('home');
export const pendingFiles = writable<string[]>([]);
export const openSession = writable<OpenSession | null>(null);
export const isLoading = writable(false);
export const loadingProgress = writable(0);
export const loadingStatus = writable('');
