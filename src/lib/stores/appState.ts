import { writable } from 'svelte/store';

export interface VaultItem {
  id: string;
  name: string;
  path: string;
  grid_x: number;
  grid_y: number;
  size_bytes: number;
  created_at: number;
  has_extra_key: boolean;
}

export interface VaultFileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
}

export interface ActiveVaultSession {
  id: string;
  name: string;
  password: string;
  is_extra: boolean;
  entries: VaultFileEntry[];
}

export const vaultsList = writable<VaultItem[]>([]);
export const activeVault = writable<ActiveVaultSession | null>(null);
export const currentModal = writable<'none' | 'create' | 'unlock' | 'extra_key' | 'settings'>('none');
export const selectedVaultId = writable<string | null>(null);
export const isProcessing = writable<boolean>(false);
export const processingStatus = writable<string>('');
