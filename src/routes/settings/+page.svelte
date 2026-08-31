<script lang="ts">
  import { t, currentLocale, type Locale } from '$lib/i18n/i18n';
  import { kdfSettings } from '$lib/stores/appState';

  let memoryMb = Math.floor($kdfSettings.m_cost / 1024);
  let iterations = $kdfSettings.t_cost;
  let threads = $kdfSettings.p_cost;

  $: {
    kdfSettings.set({
      m_cost: memoryMb * 1024,
      t_cost: iterations,
      p_cost: threads
    });
  }

  $: isBelowFloor = memoryMb < 64 || iterations < 2;
</script>

<div class="settings-container">
  <div class="settings-header">
    <h2 class="page-title">{$t('settings')}</h2>
  </div>

  <div class="settings-card">
    <h3 class="card-section-title">{$t('language')}</h3>
    <div class="lang-options">
      <button
        class="lang-btn"
        class:active={$currentLocale === 'en'}
        on:click={() => currentLocale.set('en')}
      >
        English (EN)
      </button>
      <button
        class="lang-btn"
        class:active={$currentLocale === 'es'}
        on:click={() => currentLocale.set('es')}
      >
        Español (ES)
      </button>
    </div>
  </div>

  <div class="settings-card">
    <h3 class="card-section-title">{$t('kdf_settings')}</h3>

    {#if isBelowFloor}
      <div class="warning-box">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
        <span>{$t('kdf_warning_floor')}</span>
      </div>
    {/if}

    <div class="param-row">
      <label class="param-label">{$t('kdf_memory')}:</label>
      <input type="number" min="16" max="2048" bind:value={memoryMb} class="param-input" />
    </div>

    <div class="param-row">
      <label class="param-label">{$t('kdf_iterations')}:</label>
      <input type="number" min="1" max="50" bind:value={iterations} class="param-input" />
    </div>

    <div class="param-row">
      <label class="param-label">{$t('kdf_parallelism')}:</label>
      <input type="number" min="1" max="16" bind:value={threads} class="param-input" />
    </div>
  </div>

  <div class="settings-card about-card">
    <h3 class="card-section-title">{$t('about_title')}</h3>
    <p class="about-desc">{$t('about_text')}</p>
    <div class="security-rules">
      <strong>Panic Shortcut:</strong> Press <code>Ctrl + Shift + Q</code> anytime to instantly wipe memory & reload.
    </div>
  </div>
</div>

<style>
  .settings-container {
    max-width: 600px;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .page-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f8fafc;
  }

  .settings-card {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.75rem;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .card-section-title {
    font-size: 1.05rem;
    font-weight: 600;
    color: #f8fafc;
  }

  .lang-options {
    display: flex;
    gap: 0.75rem;
  }

  .lang-btn {
    flex: 1;
    background: #090d16;
    border: 1px solid #334155;
    color: #cbd5e1;
    padding: 0.6rem;
    border-radius: 0.375rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
  }

  .lang-btn.active {
    background: #1e293b;
    border-color: #38bdf8;
    color: #38bdf8;
  }

  .warning-box {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: rgba(245, 158, 11, 0.1);
    border: 1px solid rgba(245, 158, 11, 0.3);
    color: #fbbf24;
    padding: 0.6rem 0.8rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
  }

  .param-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .param-label {
    font-size: 0.85rem;
    color: #94a3b8;
  }

  .param-input {
    width: 100px;
    background: #090d16;
    border: 1px solid #334155;
    color: #f8fafc;
    padding: 0.4rem 0.6rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    font-family: 'JetBrains Mono', monospace;
  }

  .about-desc {
    font-size: 0.85rem;
    color: #94a3b8;
    line-height: 1.5;
  }

  .security-rules {
    font-size: 0.8rem;
    color: #cbd5e1;
    background: #090d16;
    padding: 0.6rem 0.8rem;
    border-radius: 0.375rem;
    border: 1px solid #1e293b;
  }

  .security-rules code {
    color: #f87171;
    font-family: 'JetBrains Mono', monospace;
  }
</style>
