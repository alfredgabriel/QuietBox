<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { currentModal, vaultsList, isProcessing, processingStatus } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let vaultName = '';
  let password = '';
  let errorMsg = '';

  async function handleCreate() {
    if (!vaultName.trim()) {
      errorMsg = $t('enter_valid_name');
      return;
    }
    if (!password || password.length < 4) {
      errorMsg = $t('enter_valid_password');
      return;
    }

    errorMsg = '';
    isProcessing.set(true);
    processingStatus.set('Creating vault...');

    try {
      const newVault = await invoke<any>('create_vault', {
        name: vaultName.trim(),
        password: password
      });
      vaultsList.update(list => [...list, newVault]);
      currentModal.set('none');
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : 'Failed to create vault';
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
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#38bdf8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="12" y1="11" x2="12" y2="17"></line><line x1="9" y1="14" x2="15" y2="14"></line></svg>
      </div>
      <h3 class="modal-title">{$t('new_vault')}</h3>
    </div>

    {#if errorMsg}
      <div class="error-box">{errorMsg}</div>
    {/if}

    <div class="form-body">
      <div class="field">
        <label class="label">{$t('vault_name')}</label>
        <input
          type="text"
          class="input"
          bind:value={vaultName}
          placeholder={$t('enter_vault_name')}
          autofocus
        />
      </div>

      <PasswordInput
        label={$t('password')}
        bind:value={password}
        placeholder={$t('enter_password')}
      />
    </div>

    <div class="modal-footer">
      <button class="btn cancel" on:click={handleClose}>{$t('cancel')}</button>
      <button class="btn submit" on:click={handleCreate}>{$t('create')}</button>
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
    max-width: 420px;
    width: 90%;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.7);
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }
  .modal-header {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }
  .header-icon {
    width: 36px;
    height: 36px;
    background: rgba(56, 189, 248, 0.1);
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
    gap: 1rem;
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }
  .label {
    font-size: 0.85rem;
    font-weight: 500;
    color: #94a3b8;
  }
  .input {
    background: #090d16;
    border: 1px solid #334155;
    color: #f8fafc;
    padding: 0.6rem 0.85rem;
    border-radius: 0.5rem;
    font-size: 0.95rem;
    outline: none;
  }
  .input:focus {
    border-color: #38bdf8;
  }
  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 0.5rem;
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
    background: #0284c7;
    color: #ffffff;
  }
  .btn.submit:hover {
    background: #0369a1;
  }
</style>
