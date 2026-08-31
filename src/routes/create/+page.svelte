<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { currentView, progressState, kdfSettings } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import DropZone from '$lib/components/DropZone.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';
  import WarningModal from '$lib/components/WarningModal.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';

  let step = 1;
  let savePath = '';
  let totalSizeMb = 100;

  // Decoy volume
  let decoyPassword = '';
  let decoyFiles: string[] = [];

  // Hidden volume
  let addHidden = false;
  let hiddenPassword = '';
  let hiddenFiles: string[] = [];
  let hiddenSizeMb = 30;

  let showWarning = false;
  let isSubmitting = false;
  let errorMessage = '';
  let isFinished = false;

  const presets = [50, 100, 250, 500, 1000];

  onMount(() => {
    const unlisten = listen<{ progress: number; status: string }>('progress', (e) => {
      progressState.set({
        active: true,
        progress: e.payload.progress,
        status: e.payload.status
      });
    });
    return () => {
      unlisten.then(fn => fn());
    };
  });

  async function handleBrowseSave() {
    try {
      const res = await invoke<string | null>('pick_save_path');
      if (res) savePath = res;
    } catch (e) {
      console.error(e);
    }
  }

  function validateStep1(): boolean {
    if (!savePath) {
      errorMessage = 'Please select a save location.';
      return false;
    }
    if (totalSizeMb < 10) {
      errorMessage = 'Container size must be at least 10 MB.';
      return false;
    }
    errorMessage = '';
    return true;
  }

  function validateStep2(): boolean {
    if (!decoyPassword || decoyPassword.length < 4) {
      errorMessage = 'Please enter a valid decoy password (min 4 chars).';
      return false;
    }
    errorMessage = '';
    return true;
  }

  function validateStep3(): boolean {
    if (addHidden) {
      if (!hiddenPassword || hiddenPassword.length < 4) {
        errorMessage = 'Please enter a valid hidden password (min 4 chars).';
        return false;
      }
      if (hiddenPassword === decoyPassword) {
        errorMessage = 'Hidden password MUST be different from decoy password!';
        return false;
      }
      if (hiddenSizeMb >= totalSizeMb / 2) {
        errorMessage = 'Hidden size must be less than 50% of the total container size.';
        return false;
      }
    }
    errorMessage = '';
    return true;
  }

  async function executeCreation() {
    showWarning = false;
    isSubmitting = true;
    errorMessage = '';
    progressState.set({ active: true, progress: 0, status: 'Initializing container...' });

    try {
      // 1. Create decoy volume
      await invoke('create_container', {
        path: savePath,
        totalSizeMb: totalSizeMb,
        decoyPassword: decoyPassword,
        decoyFiles: decoyFiles,
        hiddenMaxSizeMb: addHidden ? hiddenSizeMb : 0,
        kdfMCost: $kdfSettings.m_cost,
        kdfTCost: $kdfSettings.t_cost,
        kdfPCost: $kdfSettings.p_cost,
      });

      // 2. Add hidden volume if requested
      if (addHidden) {
        await invoke('add_hidden_volume', {
          containerPath: savePath,
          hiddenPassword: hiddenPassword,
          hiddenFiles: hiddenFiles,
          maxHiddenSizeMb: hiddenSizeMb,
          totalSizeMb: totalSizeMb,
          kdfMCost: $kdfSettings.m_cost,
          kdfTCost: $kdfSettings.t_cost,
          kdfPCost: $kdfSettings.p_cost,
        });
      }

      isFinished = true;
      progressState.set({ active: false, progress: 1, status: 'Complete' });
    } catch (e: any) {
      errorMessage = typeof e === 'string' ? e : 'An error occurred during container creation.';
      progressState.set({ active: false, progress: 0, status: '' });
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="wizard-container">
  <div class="wizard-header">
    <div class="steps-indicator">
      <span>{$t('step')} {step} {$t('of')} 3</span>
    </div>
    <h2 class="wizard-title">{$t('create_container')}</h2>
  </div>

  {#if errorMessage}
    <div class="error-banner">{errorMessage}</div>
  {/if}

  {#if isSubmitting || $progressState.active}
    <div class="progress-card">
      <h3 class="progress-title">{$t('creating_container')}</h3>
      <ProgressBar progress={$progressState.progress} status={$progressState.status} />
    </div>
  {:else if isFinished}
    <div class="success-card">
      <div class="success-icon">
        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>
      </div>
      <h3 class="success-title">{$t('success_create')}</h3>
      <p class="success-desc">
        Your plausible-deniability container has been created at:<br />
        <code>{savePath}</code>
      </p>
      <div class="wizard-actions">
        <button class="btn primary" on:click={() => currentView.set('home')}>Return to Dashboard</button>
      </div>
    </div>
  {:else}
    <div class="wizard-card">
      {#if step === 1}
        <!-- Step 1: Location and Size -->
        <div class="form-group">
          <label class="form-label">{$t('choose_location')}</label>
          <div class="path-row">
            <input type="text" class="text-input" bind:value={savePath} placeholder="C:\path\to\vault.bin" />
            <button type="button" class="btn secondary" on:click={handleBrowseSave}>{$t('browse')}</button>
          </div>
        </div>

        <div class="form-group">
          <label class="form-label">{$t('total_size')}</label>
          <div class="preset-row">
            {#each presets as sz}
              <button
                type="button"
                class="preset-btn"
                class:active={totalSizeMb === sz}
                on:click={() => (totalSizeMb = sz)}
              >
                {sz >= 1000 ? `${sz / 1000} GB` : `${sz} MB`}
              </button>
            {/each}
          </div>
          <div class="custom-size-row">
            <span class="sub-label">{$t('custom_size')}:</span>
            <input type="number" min="10" max="50000" class="num-input" bind:value={totalSizeMb} />
          </div>
          <p class="hint-text">{$t('size_explanation')}</p>
        </div>

        <div class="wizard-actions">
          <button class="btn secondary" on:click={() => currentView.set('home')}>{$t('cancel')}</button>
          <button
            class="btn primary"
            on:click={() => {
              if (validateStep1()) step = 2;
            }}
          >
            {$t('next')} &rarr;
          </button>
        </div>

      {:else if step === 2}
        <!-- Step 2: Decoy Volume -->
        <div class="section-intro">
          <h3 class="section-title">{$t('decoy_volume_title')}</h3>
          <p class="hint-text">{$t('decoy_explanation')}</p>
        </div>

        <PasswordInput
          label={$t('decoy_password')}
          bind:value={decoyPassword}
          placeholder="Enter innocuous decoy password"
        />

        <DropZone
          title={$t('decoy_files')}
          bind:files={decoyFiles}
        />

        <div class="wizard-actions">
          <button class="btn secondary" on:click={() => (step = 1)}>{$t('back')}</button>
          <button
            class="btn primary"
            on:click={() => {
              if (validateStep2()) step = 3;
            }}
          >
            {$t('next')} &rarr;
          </button>
        </div>

      {:else if step === 3}
        <!-- Step 3: Hidden Volume & Confirmation -->
        <div class="toggle-group">
          <label class="toggle-label">
            <input type="checkbox" bind:checked={addHidden} />
            <span>{$t('hidden_volume_toggle')}</span>
          </label>
        </div>

        {#if addHidden}
          <div class="hidden-section">
            <div class="section-intro">
              <h3 class="section-title">{$t('hidden_volume_title')}</h3>
              <p class="hint-text">{$t('hidden_explanation')}</p>
            </div>

            <PasswordInput
              label={$t('hidden_password')}
              bind:value={hiddenPassword}
              placeholder="Enter top-secret hidden password"
            />

            <div class="form-group">
              <label class="form-label">{$t('hidden_reserved_size')} (Max: {Math.floor(totalSizeMb / 2)} MB)</label>
              <input
                type="range"
                min="5"
                max={Math.floor(totalSizeMb / 2) - 5}
                bind:value={hiddenSizeMb}
                class="range-slider"
              />
              <span class="range-val">{hiddenSizeMb} MB</span>
            </div>

            <DropZone
              title={$t('hidden_files')}
              bind:files={hiddenFiles}
            />
          </div>
        {/if}

        <div class="wizard-actions">
          <button class="btn secondary" on:click={() => (step = 2)}>{$t('back')}</button>
          <button
            class="btn primary create-now"
            on:click={() => {
              if (validateStep3()) {
                if (addHidden) {
                  showWarning = true;
                } else {
                  executeCreation();
                }
              }
            }}
          >
            Create Encrypted Vault
          </button>
        </div>
      {/if}
    </div>
  {/if}
</div>

<WarningModal
  show={showWarning}
  onConfirm={executeCreation}
  onCancel={() => (showWarning = false)}
/>

<style>
  .wizard-container {
    max-width: 650px;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .wizard-header {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .steps-indicator {
    font-size: 0.8rem;
    font-weight: 600;
    color: #38bdf8;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .wizard-title {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f8fafc;
  }

  .error-banner {
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid #ef4444;
    color: #fca5a5;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.85rem;
  }

  .wizard-card {
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

  .preset-row {
    display: flex;
    gap: 0.5rem;
  }

  .preset-btn {
    flex: 1;
    background: #090d16;
    border: 1px solid #334155;
    color: #cbd5e1;
    padding: 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .preset-btn.active {
    background: #1e293b;
    border-color: #38bdf8;
    color: #38bdf8;
    font-weight: 600;
  }

  .custom-size-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-top: 0.4rem;
  }

  .sub-label {
    font-size: 0.8rem;
    color: #94a3b8;
  }

  .num-input {
    width: 100px;
    background: #090d16;
    border: 1px solid #334155;
    color: #f8fafc;
    padding: 0.35rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.85rem;
  }

  .hint-text {
    font-size: 0.8rem;
    color: #64748b;
    line-height: 1.4;
  }

  .section-intro {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .section-title {
    font-size: 1.05rem;
    font-weight: 600;
    color: #f8fafc;
  }

  .toggle-group {
    background: #090d16;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    border: 1px solid #1e293b;
  }

  .toggle-label {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    font-size: 0.9rem;
    font-weight: 600;
    color: #38bdf8;
    cursor: pointer;
  }

  .toggle-label input {
    width: 18px;
    height: 18px;
    accent-color: #38bdf8;
  }

  .hidden-section {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    background: rgba(2, 132, 199, 0.05);
    border: 1px solid rgba(56, 189, 248, 0.2);
    border-radius: 0.5rem;
    padding: 1rem;
  }

  .range-slider {
    width: 100%;
    accent-color: #38bdf8;
  }

  .range-val {
    font-size: 0.85rem;
    font-weight: 600;
    color: #38bdf8;
    font-family: 'JetBrains Mono', monospace;
  }

  .wizard-actions {
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

  .btn.create-now {
    background: linear-gradient(135deg, #0284c7, #0d9488);
  }

  .progress-card, .success-card {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.75rem;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 1rem;
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
