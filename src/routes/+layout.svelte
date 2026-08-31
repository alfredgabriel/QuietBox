<script lang="ts">
  import '../app.css';
  import GlobalLoader from '$lib/components/GlobalLoader.svelte';
  import { t, currentLocale, type Locale } from '$lib/i18n/i18n';

  const langs = [
    { code: 'en', label: 'English' },
    { code: 'es', label: 'Español' },
    { code: 'fr', label: 'Français' },
    { code: 'de', label: 'Deutsch' },
  ];
</script>

<GlobalLoader />

<div class="shell">
  <div class="title-bar">
    <div class="brand">
      <img src="/favicon.png" alt="QB" width="18" height="18" class="logo" />
      <span>QuietBox</span>
    </div>
    <div class="title-right">
      <select
        class="lang-select"
        value={$currentLocale}
        on:change={(e) => currentLocale.set(e.currentTarget.value as Locale)}
        aria-label="Language"
      >
        {#each langs as l}
          <option value={l.code}>{l.label}</option>
        {/each}
      </select>
    </div>
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
  .title-right { display: flex; align-items: center; }
  .lang-select {
    background: #0b1120;
    border: 1px solid #2a4a72;
    color: #94aabe;
    padding: .22rem .5rem;
    border-radius: 5px;
    font-size: .78rem;
    font-family: 'Inter', sans-serif;
    cursor: pointer;
    outline: none;
    appearance: none;
    -webkit-appearance: none;
    padding-right: 1.4rem;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath d='M1 1l4 4 4-4' stroke='%234a6580' stroke-width='1.5' fill='none' stroke-linecap='round'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right .4rem center;
    min-width: 90px;
  }
  .lang-select:hover { border-color: #38bdf8; color: #f0f6ff; }
  .lang-select:focus { border-color: #38bdf8; }
  .lang-select option { background: #0b1120; color: #f0f6ff; }
  .viewport { flex: 1; overflow: hidden; position: relative; }
</style>
