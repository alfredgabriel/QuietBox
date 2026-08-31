<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { currentModal, selectedVaultId, vaultsList, activeVault, isProcessing, processingStatus } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let password = '';
  let errorMsg = '';

  $: targetVault = $vaultsList.find(v => v.id === $selectedVaultId);

  async function handleUnlock() {
    if (!targetVault) return;
    if (!password) {
      errorMsg = $t('enter_valid_password');
      return;
    }

    errorMsg = '';
    isProcessing.set(true);
    processingStatus.set('Unlocking vault...');

    try {
      const res = await invoke<any>('open_vault_by_id', {
        id: targetVault.id,
        password: password
      });

      activeVault.set({
        id: targetVault.id,
        name: targetVault.name,
        password: password,
        is_extra: res.is_extra,
        entries: res.entries
      });

      currentModal.set('none');
    } catch (e: any) {
      errorMsg = $t('invalid_password');
    } finally {
      isProcessing.set(false);
    }
  }

  function handleClose() {
    currentModal.set('none');
  }
</script>

<div class="modal-backdrop" on:click|self={handleClose}>
  <div class="modal-card">
    <div class="modal-header">
      <div class="header-icon">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>
      </div>
      <div>
        <h3 class="modal-title">{$t('unlock_vault')}</h3>
        <span class="subtitle">{targetVault?.name || 'Vault'}</span>
      </div>
    </div>

    {#if errorMsg}
      <div class="error-box">{errorMsg}</div>
    {/if}

    <div class="form-body">
      <PasswordInput
        label={$t('password')}
        bind:value={password}
        showStrength={false}
        placeholder={$t('enter_password')}
      />
    </div>

    <div class="modal-footer">
      <button class="btn cancel" on:click={handleClose}>{$t('cancel')}</button>
      <button class="btn submit" on:click={handleUnlock}>{$t('unlock')}</button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }
  .modal-card {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.75rem;
    padding: 1.5rem;
    max-width: 400px;
    width: 90%;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }
  .modal-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .header-icon {
    width: 40px;
    height: 40px;
    background: rgba(16, 185, 129, 0.1);
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .modal-title {
    font-size: 1.15rem;
    font-weight: 700;
    color: #f8fafc;
  }
  .subtitle {
    font-size: 0.8rem;
    color: #38bdf8;
    font-family: 'JetBrains Mono', monospace;
  }
  .error-box {
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid #ef4444;
    color: #fca5a5;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
  }
  .form-body {
    display: flex;
    flex-direction: column;
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }
  .btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.15s;
  }
  .btn.cancel {
    background: #1e293b;
    color: #94a3b8;
  }
  .btn.cancel:hover {
    background: #334155;
    color: #f8fafc;
  }
  .btn.submit {
    background: #10b981;
    color: #0f172a;
  }
  .btn.submit:hover {
    background: #059669;
  }
</style>
