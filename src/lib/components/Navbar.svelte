<script lang="ts">
  import { t, currentLocale } from '$lib/i18n/i18n';
  import { currentModal } from '$lib/stores/appState';

  function toggleLocale() {
    currentLocale.update(l => (l === 'es' ? 'en' : 'es'));
  }
</script>

<header class="app-header">
  <div class="brand">
    <div class="logo-box">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="3" y="11" width="18" height="11" rx="2" ry="2" fill="#0f172a" stroke="#38bdf8"></rect>
        <path d="M7 11V7a5 5 0 0 1 10 0v4" stroke="#38bdf8"></path>
        <circle cx="12" cy="16" r="1.5" fill="#38bdf8"></circle>
      </svg>
    </div>
    <span class="brand-text">QuietBox</span>
  </div>

  <div class="header-actions">
    <button class="btn new-vault-btn" on:click={() => currentModal.set('create')}>
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"></path></svg>
      <span>{$t('new_vault')}</span>
    </button>
    <button class="btn lang-btn" on:click={toggleLocale} title="Change language">
      {$currentLocale.toUpperCase()}
    </button>
  </div>
</header>

<style>
  .app-header {
    height: 48px;
    background: #0f172a;
    border-bottom: 1px solid #1e293b;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 1rem;
    user-select: none;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .logo-box {
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .brand-text {
    font-weight: 700;
    font-size: 1rem;
    letter-spacing: -0.02em;
    color: #f8fafc;
  }
  .header-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .btn {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.35rem 0.65rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid #334155;
    background: #1e293b;
    color: #f1f5f9;
    transition: all 0.15s;
  }
  .btn:hover {
    background: #334155;
  }
  .btn.new-vault-btn {
    background: #0284c7;
    border-color: #0369a1;
    color: #ffffff;
  }
  .btn.new-vault-btn:hover {
    background: #0369a1;
  }
  .btn.lang-btn {
    font-size: 0.75rem;
  }
</style>
