<script lang="ts">
  import { isLoading, loadingProgress, loadingStatus } from '$lib/stores/appState';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<{ progress: number; status: string }>('progress', (e) => {
      loadingProgress.set(e.payload.progress);
      loadingStatus.set(e.payload.status);
    });
  });

  onDestroy(() => unlisten?.());
</script>

{#if $isLoading}
  <div class="loader-overlay">
    <div class="loader-box">
      <div class="lock-anim">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="#38bdf8"
             stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
          <circle cx="12" cy="16" r="1.5" fill="#38bdf8" stroke="none"/>
        </svg>
      </div>
      <p class="status">{$loadingStatus || 'Processing…'}</p>
      <div class="bar-track">
        <div class="bar-fill" style="width:{Math.round($loadingProgress * 100)}%"></div>
      </div>
      <span class="pct">{Math.round($loadingProgress * 100)}%</span>
    </div>
  </div>
{/if}

<style>
  .loader-overlay {
    position: fixed; inset: 0;
    background: rgba(6, 11, 20, .85);
    backdrop-filter: blur(8px);
    display: flex; align-items: center; justify-content: center;
    z-index: 9999;
  }
  .loader-box {
    background: #0f172a;
    border: 1px solid #1e3050;
    border-radius: 16px;
    padding: 2rem 2.5rem;
    min-width: 280px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    box-shadow: 0 24px 64px rgba(0,0,0,.7);
  }
  .lock-anim {
    animation: pulse 1.8s ease-in-out infinite;
    filter: drop-shadow(0 0 12px #38bdf8aa);
  }
  @keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50%       { opacity: .6; transform: scale(.92); }
  }
  .status {
    font-size: .88rem;
    color: #94aabe;
    text-align: center;
  }
  .bar-track {
    width: 100%;
    height: 5px;
    background: #1e3050;
    border-radius: 99px;
    overflow: hidden;
  }
  .bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #0ea5e9, #38bdf8);
    border-radius: 99px;
    transition: width .2s ease;
  }
  .pct {
    font-family: 'JetBrains Mono', monospace;
    font-size: .78rem;
    color: #38bdf8;
  }
</style>
