<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { currentView, progressState, kdfSettings } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let containerPath = '';
  let password = '';
  let outputDir = '';

  let isUnlocking = false;
  let errorMessage = '';
  let isUnlocked = false;

  async function handleBrowseContainer() {
    try {
      const res = await invoke<string | null>('pick_file');
      if (res) containerPath = res;
    } catch (e) {
      console.error(e);
    }
  }

  async function handleBrowseOutputDir() {
    try {
      const res = await invoke<string | null>('pick_directory');
      if (res) outputDir = res;
    } catch (e) {
      console.error(e);
    }
  }

  async function handleUnlock() {
    if (!containerPath) {
      errorMessage = 'Please select a container file.';
      return;
    }
    if (!password) {
      errorMessage = 'Please enter the password.';
      return;
    }
    if (!outputDir) {
      errorMessage = 'Please select an extraction output folder.';
      return;
    }

    errorMessage = '';
    isUnlocking = true;
    progressState.set({ active: true, progress: 0.5, status: 'Verifying keys & decrypting...' });

    try {
      await invoke<string>('open_container', {
        containerPath,
        password,
        outputDir,
        kdfMCost: $kdfSettings.m_cost,
        kdfTCost: $kdfSettings.t_cost,
        kdfPCost: $kdfSettings.p_cost,
      });

      isUnlocked = true;
      progressState.set({ active: false, progress: 1, status: 'Decrypted' });
    } catch (e: any) {
      errorMessage = typeof e === 'string' ? e : 'Invalid password or corrupted container.';
      progressState.set({ active: false, progress: 0, status: '' });
    } finally {
      isUnlocking = false;
    }
  }
</script>

<div class="open-container">
  <div class="open-header">
    <h2 class="page-title">{$t('open_container')}</h2>
    <p class="page-subtitle">{$t('password_hint')}</p>
  </div>

  {#if errorMessage}
    <div class="error-banner">{errorMessage}</div>
  {/if}

  {#if isUnlocking || $progressState.active}
    <div class="card progress-card">
      <h3 class="progress-title">{$t('opening_container')}</h3>
      <ProgressBar progress={$progressState.progress} status={$progressState.status} />
    </div>
  {:else if isUnlocked}
    <div class="card success-card">
      <div class="success-icon">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
      </div>
      <h3 class="success-title">{$t('success_open')}</h3>
      <p class="success-desc">
        {$t('files_extracted')}<br />
        <code>{outputDir}</code>
      </p>
      <div class="actions">
        <button class="btn primary" on:click={() => currentView.set('home')}>Return to Dashboard</button>
      </div>
    </div>
  {:else}
    <div class="card form-card">
      <div class="form-group">
        <label class="form-label">{$t('container_path')}</label>
        <div class="path-row">
          <input type="text" class="text-input" bind:value={containerPath} placeholder="Select container file..." />
          <button type="button" class="btn secondary" on:click={handleBrowseContainer}>{$t('browse')}</button>
        </div>
      </div>

      <div class="form-group">
        <label class="form-label">{$t('extract_to')}</label>
        <div class="path-row">
          <input type="text" class="text-input" bind:value={outputDir} placeholder="Select destination folder..." />
          <button type="button" class="btn secondary" on:click={handleBrowseOutputDir}>{$t('browse')}</button>
        </div>
      </div>

      <PasswordInput
        label={$t('enter_password')}
        bind:value={password}
        showStrength={false}
        placeholder="Enter password..."
      />

      <div class="actions">
        <button class="btn secondary" on:click={() => currentView.set('home')}>{$t('cancel')}</button>
        <button class="btn primary unlock-btn" on:click={handleUnlock}>
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
          <span>{$t('unlock')}</span>
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .open-container {
    max-width: 580px;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .open-header {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .page-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f8fafc;
  }

  .page-subtitle {
    font-size: 0.85rem;
    color: #94a3b8;
  }

  .error-banner {
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid #ef4444;
    color: #fca5a5;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.85rem;
  }

  .card {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.75rem;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .form-label {
    font-size: 0.85rem;
    font-weight: 500;
    color: #94a3b8;
  }

  .path-row {
    display: flex;
    gap: 0.5rem;
  }

  .text-input {
    flex: 1;
    background: #090d16;
    border: 1px solid #334155;
    color: #f8fafc;
    padding: 0.6rem 0.8rem;
    border-radius: 0.375rem;
    font-size: 0.9rem;
    outline: none;
  }

  .text-input:focus {
    border-color: #38bdf8;
  }

  .actions {
    display: flex;
    justify-content: space-between;
    margin-top: 0.5rem;
  }

  .btn {
    padding: 0.6rem 1.25rem;
    border-radius: 0.375rem;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.15s;
  }

  .btn.secondary {
    background: #1e293b;
    color: #cbd5e1;
    border: 1px solid #334155;
  }

  .btn.secondary:hover {
    background: #334155;
  }

  .btn.primary {
    background: #0284c7;
    color: #ffffff;
  }

  .btn.primary:hover {
    background: #0369a1;
  }

  .btn.unlock-btn {
    background: linear-gradient(135deg, #0284c7, #10b981);
  }

  .progress-card, .success-card {
    text-align: center;
    align-items: center;
    padding: 2rem;
  }

  .success-icon {
    width: 56px;
    height: 56px;
    background: rgba(16, 185, 129, 0.15);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .success-title {
    font-size: 1.35rem;
    font-weight: 700;
    color: #f8fafc;
  }

  .success-desc {
    color: #94a3b8;
    font-size: 0.9rem;
  }

  .success-desc code {
    display: inline-block;
    margin-top: 0.5rem;
    background: #090d16;
    padding: 0.3rem 0.6rem;
    border-radius: 0.25rem;
    font-family: 'JetBrains Mono', monospace;
    color: #38bdf8;
  }
</style>
