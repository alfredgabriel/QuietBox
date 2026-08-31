<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { currentModal, activeVault, isProcessing, processingStatus } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let extraPassword = '';
  let errorMsg = '';
  let successMsg = '';

  async function handleSaveExtra() {
    if (!$activeVault) return;
    if (!extraPassword || extraPassword.length < 4) {
      errorMsg = $t('enter_valid_password');
      return;
    }
    if (extraPassword === $activeVault.password) {
      errorMsg = $t('passwords_must_differ');
      return;
    }

    errorMsg = '';
    isProcessing.set(true);
    processingStatus.set('Configuring extra security space...');

    try {
      await invoke('add_extra_password', {
        id: $activeVault.id,
        extraPassword: extraPassword
      });
      successMsg = $t('key_saved');
      setTimeout(() => {
        currentModal.set('none');
      }, 1200);
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : 'Failed to configure extra key.';
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
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><key><circle cx="7.5" cy="15.5" r="5.5"></circle><path d="m21 2-9.6 9.6"></path><path d="m15.5 7.5 3 3L22 7l-3-3"></path></key></svg>
      </div>
      <h3 class="modal-title">{$t('extra_key')}</h3>
    </div>

    <p class="explanation">{$t('extra_key_explanation')}</p>

    {#if errorMsg}
      <div class="error-box">{errorMsg}</div>
    {/if}
    {#if successMsg}
      <div class="success-box">{successMsg}</div>
    {/if}

    <div class="form-body">
      <PasswordInput
        label={$t('extra_key_password')}
        bind:value={extraPassword}
        placeholder="••••••••••••"
      />
    </div>

    <div class="modal-footer">
      <button class="btn cancel" on:click={handleClose}>{$t('cancel')}</button>
      <button class="btn submit" on:click={handleSaveExtra}>{$t('save_key')}</button>
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
    gap: 1rem;
  }
  .modal-header {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }
  .header-icon {
    width: 36px;
    height: 36px;
    background: rgba(245, 158, 11, 0.15);
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
  .explanation {
    font-size: 0.85rem;
    color: #94a3b8;
    line-height: 1.4;
  }
  .error-box {
    background: rgba(239, 68, 68, 0.15);
    border: 1px solid #ef4444;
    color: #fca5a5;
    padding: 0.5rem 0.75rem;
    border-radius: 0.375rem;
    font-size: 0.8rem;
  }
  .success-box {
    background: rgba(16, 185, 129, 0.15);
    border: 1px solid #10b981;
    color: #6ee7b7;
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
  }
  .btn.cancel {
    background: #1e293b;
    color: #94a3b8;
  }
  .btn.submit {
    background: #f59e0b;
    color: #0f172a;
  }
</style>
