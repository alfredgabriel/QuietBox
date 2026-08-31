<script lang="ts">
  import { t } from '$lib/i18n/i18n';

  export let show: boolean = false;
  export let onConfirm: () => void = () => {};
  export let onCancel: () => void = () => {};
</script>

{#if show}
  <div class="modal-backdrop">
    <div class="modal-card">
      <div class="icon-wrap">
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"></path><line x1="12" y1="9" x2="12" y2="13"></line><line x1="12" y1="17" x2="12.01" y2="17"></line></svg>
      </div>
      <h3 class="title">{$t('security_warnings')}</h3>
      <ul class="warning-list">
        <li><strong>{$t('warning_no_modify_decoy')}</strong></li>
        <li>{$t('warning_cloud_sync')}</li>
        <li>{$t('warning_password_entropy')}</li>
      </ul>
      <div class="modal-actions">
        <button class="btn cancel" on:click={onCancel}>{$t('cancel')}</button>
        <button class="btn confirm" on:click={onConfirm}>I Understand & Proceed</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.75);
    backdrop-filter: blur(4px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .modal-card {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.75rem;
    padding: 1.5rem;
    max-width: 480px;
    width: 90%;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .icon-wrap {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    background: rgba(245, 158, 11, 0.1);
    border-radius: 50%;
  }

  .title {
    font-size: 1.15rem;
    font-weight: 600;
    color: #f8fafc;
    margin: 0;
  }

  .warning-list {
    margin: 0;
    padding-left: 1.25rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    font-size: 0.85rem;
    color: #cbd5e1;
    line-height: 1.4;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 0.5rem;
  }

  .btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: background 0.15s;
  }

  .btn.cancel {
    background: #1e293b;
    color: #94a3b8;
  }

  .btn.cancel:hover {
    background: #334155;
    color: #f8fafc;
  }

  .btn.confirm {
    background: #f59e0b;
    color: #0f172a;
    font-weight: 600;
  }

  .btn.confirm:hover {
    background: #d97706;
  }
</style>
