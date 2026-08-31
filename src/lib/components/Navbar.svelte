<script lang="ts">
  import { t, currentLocale, type Locale } from '$lib/i18n/i18n';
  import { currentView, panicWipe, type AppView } from '$lib/stores/appState';

  function setView(v: AppView) {
    currentView.set(v);
  }

  function toggleLocale() {
    currentLocale.update(l => (l === 'es' ? 'en' : 'es'));
  }
</script>

<nav class="navbar">
  <div class="brand" on:click={() => setView('home')}>
    <div class="logo-mark">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
    </div>
    <span class="app-title">QuietBox</span>
  </div>

  <div class="nav-links">
    <button class="nav-btn" class:active={$currentView === 'home'} on:click={() => setView('home')}>
      {$t('dashboard')}
    </button>
    <button class="nav-btn" class:active={$currentView === 'create'} on:click={() => setView('create')}>
      {$t('create_container')}
    </button>
    <button class="nav-btn" class:active={$currentView === 'open'} on:click={() => setView('open')}>
      {$t('open_container')}
    </button>
    <button class="nav-btn" class:active={$currentView === 'settings'} on:click={() => setView('settings')}>
      {$t('settings')}
    </button>
  </div>

  <div class="nav-actions">
    <button class="lang-toggle" on:click={toggleLocale} title="Change language">
      {$currentLocale.toUpperCase()}
    </button>
    <button class="panic-btn" on:click={panicWipe} title={$t('panic_button')}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polygon points="7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"></polygon><line x1="12" y1="8" x2="12" y2="12"></line><line x1="12" y1="16" x2="12.01" y2="16"></line></svg>
      <span>Panic</span>
    </button>
  </div>
</nav>

<style>
  .navbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1.25rem;
    background: #090d16;
    border-bottom: 1px solid #1e293b;
    user-select: none;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    cursor: pointer;
  }

  .logo-mark {
    width: 28px;
    height: 28px;
    background: linear-gradient(135deg, #0284c7, #0369a1);
    color: white;
    border-radius: 0.375rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .app-title {
    font-weight: 700;
    font-size: 1.05rem;
    letter-spacing: -0.02em;
    color: #f8fafc;
  }

  .nav-links {
    display: flex;
    gap: 0.25rem;
    background: #0f172a;
    padding: 0.25rem;
    border-radius: 0.5rem;
    border: 1px solid #1e293b;
  }

  .nav-btn {
    background: none;
    border: none;
    color: #94a3b8;
    padding: 0.35rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
  }

  .nav-btn:hover {
    color: #f1f5f9;
  }

  .nav-btn.active {
    background: #1e293b;
    color: #38bdf8;
    font-weight: 600;
  }

  .nav-actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .lang-toggle {
    background: #0f172a;
    border: 1px solid #1e293b;
    color: #cbd5e1;
    padding: 0.35rem 0.6rem;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 700;
    cursor: pointer;
  }

  .lang-toggle:hover {
    border-color: #334155;
  }

  .panic-btn {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid rgba(239, 68, 68, 0.3);
    color: #f87171;
    padding: 0.35rem 0.65rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s;
  }

  .panic-btn:hover {
    background: rgba(239, 68, 68, 0.25);
    border-color: #ef4444;
  }
</style>
