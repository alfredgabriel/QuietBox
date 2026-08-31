<script lang="ts">
  import '../app.css';
  import GlobalLoader from '$lib/components/GlobalLoader.svelte';
  import { t, currentLocale } from '$lib/i18n/i18n';
</script>

<GlobalLoader />

<div class="shell">
  <!-- Compact title bar -->
  <div class="title-bar">
    <div class="brand">
      <img src="/favicon.png" alt="QB" width="18" height="18" class="logo" />
      <span>QuietBox</span>
    </div>
    <button class="lang-btn" on:click={() => currentLocale.update(l => l === 'es' ? 'en' : 'es')}>
      {$currentLocale.toUpperCase()}
    </button>
  </div>

  <main class="viewport">
    <slot />
  </main>
</div>

<style>
  .shell { display: flex; flex-direction: column; height: 100vh; overflow: hidden; }
  .title-bar {
    height: 40px;
    background: #060b14;
    border-bottom: 1px solid #1e3050;
    display: flex; align-items: center; justify-content: space-between;
    padding: 0 .85rem;
    flex-shrink: 0;
  }
  .brand { display: flex; align-items: center; gap: .45rem; font-size: .88rem; font-weight: 700; color: #f0f6ff; }
  .logo { border-radius: 4px; }
  .lang-btn {
    background: #1e3050; border: 1px solid #2a4a72;
    color: #94aabe; padding: .2rem .5rem;
    border-radius: 5px; font-size: .72rem; font-weight: 700;
    cursor: pointer; font-family: 'JetBrains Mono', monospace;
  }
  .lang-btn:hover { background: #2a4a72; color: #f0f6ff; }
  .viewport { flex: 1; overflow: hidden; position: relative; }
</style>
