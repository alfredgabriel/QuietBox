<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { createEventDispatcher } from 'svelte';
  import PasswordInput from '$lib/components/PasswordInput.svelte';

  export let archiveName = '';
  export let errorMsg = '';

  const dispatch = createEventDispatcher();
  let password = '';

  function submit() {
    if (!password) return;
    dispatch('unlock', password);
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Enter') submit();
    if (e.key === 'Escape') dispatch('cancel');
  }
</script>

<svelte:window on:keydown={onKey}/>

<div class="backdrop" role="dialog" aria-modal="true">
  <div class="dlg">
    <div class="dlg-head">
      <div class="lock-icon">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
          <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
        </svg>
      </div>
      <div>
        <h3>{$t('unlock')}</h3>
        <span class="sub">{archiveName}</span>
      </div>
    </div>

    {#if errorMsg}
      <div class="err">{errorMsg || $t('wrong_password')}</div>
    {/if}

    <PasswordInput
      bind:value={password}
      placeholder={$t('enter_password')}
      showStrength={false}
    />

    <div class="dlg-foot">
      <button class="btn btn-ghost" on:click={() => dispatch('cancel')}>{$t('cancel')}</button>
      <button class="btn btn-primary" on:click={submit}>{$t('unlock')}</button>
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed; inset: 0;
    background: rgba(6,11,20,.8);
    backdrop-filter: blur(6px);
    display: flex; align-items: center; justify-content: center;
    z-index: 500;
  }
  .dlg {
    background: #0f172a;
    border: 1px solid #1e3050;
    border-radius: 14px;
    padding: 1.5rem;
    width: 360px;
    display: flex; flex-direction: column; gap: 1rem;
    box-shadow: 0 24px 64px rgba(0,0,0,.7);
  }
  .dlg-head { display: flex; align-items: center; gap: .75rem; }
  .lock-icon {
    width: 40px; height: 40px;
    background: rgba(16,185,129,.1);
    border-radius: 10px;
    display: flex; align-items: center; justify-content: center;
    flex-shrink: 0;
  }
  h3 { font-size: 1.05rem; font-weight: 700; color: #f0f6ff; }
  .sub { font-size: .78rem; color: #38bdf8; font-family: 'JetBrains Mono', monospace; }
  .err {
    background: rgba(239,68,68,.12);
    border: 1px solid rgba(239,68,68,.3);
    color: #f87171;
    padding: .5rem .75rem;
    border-radius: 6px;
    font-size: .82rem;
  }
  .dlg-foot { display: flex; justify-content: flex-end; gap: .6rem; }
</style>
