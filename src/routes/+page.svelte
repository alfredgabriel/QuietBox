<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { currentView } from '$lib/stores/appState';
  import CreatePage from './create/+page.svelte';
  import OpenPage from './open/+page.svelte';
  import SettingsPage from './settings/+page.svelte';
</script>

{#if $currentView === 'home'}
  <div class="dashboard-container">
    <div class="hero">
      <div class="badge">Plausible Deniability Security</div>
      <h1 class="hero-title">QuietBox</h1>
      <p class="hero-subtitle">{$t('app_tagline')}</p>
    </div>

    <div class="action-cards">
      <div class="card" on:click={() => currentView.set('create')}>
        <div class="card-icon blue">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"></path></svg>
        </div>
        <h3 class="card-title">{$t('create_container')}</h3>
        <p class="card-desc">Create a new fixed-size encrypted container with a decoy and optional hidden volume.</p>
        <button class="card-btn">Get Started &rarr;</button>
      </div>

      <div class="card" on:click={() => currentView.set('open')}>
        <div class="card-icon emerald">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 9.9-1"></path></svg>
        </div>
        <h3 class="card-title">{$t('open_container')}</h3>
        <p class="card-desc">Unlock a container with your password. Automatically reveals decoy or hidden content without clues.</p>
        <button class="card-btn">Unlock Volume &rarr;</button>
      </div>
    </div>

    <div class="security-banner">
      <div class="shield-icon">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#38bdf8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path></svg>
      </div>
      <div class="banner-text">
        <strong>Mathematically Indistinguishable from Pure Noise:</strong> Containers have 0 plaintext headers, no magic bytes, and are packed with CSPRNG entropy.
      </div>
    </div>
  </div>
{:else if $currentView === 'create'}
  <CreatePage />
{:else if $currentView === 'open'}
  <OpenPage />
{:else if $currentView === 'settings'}
  <SettingsPage />
{/if}

<style>
  .dashboard-container {
    max-width: 800px;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 2rem;
    margin-top: 1rem;
  }

  .hero {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
  }

  .badge {
    background: rgba(56, 189, 248, 0.1);
    color: #38bdf8;
    border: 1px solid rgba(56, 189, 248, 0.25);
    font-size: 0.75rem;
    font-weight: 600;
    padding: 0.25rem 0.65rem;
    border-radius: 9999px;
  }

  .hero-title {
    font-size: 2.25rem;
    font-weight: 800;
    letter-spacing: -0.03em;
    color: #f8fafc;
  }

  .hero-subtitle {
    color: #94a3b8;
    font-size: 1rem;
    max-width: 500px;
  }

  .action-cards {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.25rem;
  }

  .card {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.75rem;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .card:hover {
    border-color: #38bdf8;
    transform: translateY(-2px);
    box-shadow: 0 10px 20px -5px rgba(0, 0, 0, 0.4);
  }

  .card-icon {
    width: 44px;
    height: 44px;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .card-icon.blue {
    background: rgba(56, 189, 248, 0.15);
    color: #38bdf8;
  }

  .card-icon.emerald {
    background: rgba(16, 185, 129, 0.15);
    color: #10b981;
  }

  .card-title {
    font-size: 1.15rem;
    font-weight: 600;
    color: #f8fafc;
  }

  .card-desc {
    font-size: 0.85rem;
    color: #94a3b8;
    line-height: 1.4;
    flex: 1;
  }

  .card-btn {
    align-self: flex-start;
    background: none;
    border: none;
    color: #38bdf8;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    padding: 0;
    margin-top: 0.5rem;
  }

  .security-banner {
    display: flex;
    align-items: center;
    gap: 0.85rem;
    background: rgba(15, 23, 42, 0.8);
    border: 1px solid #1e293b;
    border-radius: 0.5rem;
    padding: 1rem 1.25rem;
  }

  .banner-text {
    font-size: 0.82rem;
    color: #94a3b8;
    line-height: 1.4;
  }

  .banner-text strong {
    color: #f1f5f9;
  }
</style>
