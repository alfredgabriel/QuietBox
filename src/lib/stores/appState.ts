import { writable } from 'svelte/store';

export type AppView = 'home' | 'create' | 'open' | 'settings';

export interface ProgressState {
  active: boolean;
  progress: number;
  status: string;
}

export const currentView = writable<AppView>('home');
export const progressState = writable<ProgressState>({
  active: false,
  progress: 0,
  status: ''
});

export const kdfSettings = writable({
  m_cost: 262144, // 256 MB
  t_cost: 3,
  p_cost: 4
});

export function panicWipe() {
  currentView.set('home');
  progressState.set({ active: false, progress: 0, status: '' });
  // Overwrite sensitive form fields or wipe session memory
  window.location.reload();
}
