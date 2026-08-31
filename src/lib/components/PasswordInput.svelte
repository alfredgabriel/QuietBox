<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  export let value = '';
  export let label = '';
  export let placeholder = '••••••••';
  export let showStrength = true;
  export let id = 'pwd-' + Math.random().toString(36).slice(2);

  let visible = false;

  $: strength = (() => {
    if (value.length === 0) return 0;
    let s = 0;
    if (value.length >= 8) s++;
    if (value.length >= 12) s++;
    if (/[A-Z]/.test(value)) s++;
    if (/[0-9]/.test(value)) s++;
    if (/[^A-Za-z0-9]/.test(value)) s++;
    return Math.min(4, Math.ceil(s * 4 / 5));
  })();

  const labels = ['', 'pwd_weak', 'pwd_fair', 'pwd_good', 'pwd_strong'];
  const colors = ['', '#ef4444', '#f59e0b', '#38bdf8', '#10b981'];
</script>

<div class="pwd-field section">
  {#if label}
    <label class="label" for={id}>{label}</label>
  {/if}
  <div class="wrapper">
    {#if visible}
      <input {id} type="text" class="input" {placeholder} bind:value />
    {:else}
      <input {id} type="password" class="input" {placeholder} bind:value />
    {/if}
    <button type="button" class="toggle" on:click={() => visible = !visible}
      aria-label="Toggle password visibility">
      {#if visible}
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"/><line x1="1" y1="1" x2="23" y2="23"/></svg>
      {:else}
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>
      {/if}
    </button>
  </div>
  {#if showStrength && value}
    <div class="strength-row">
      {#each [1,2,3,4] as i}
        <div class="seg" style="background:{i<=strength ? colors[strength] : '#1e3050'}"></div>
      {/each}
      <span class="str-label" style="color:{colors[strength]}">{$t(labels[strength])}</span>
    </div>
  {/if}
</div>

<style>
  .wrapper { position: relative; }
  .toggle {
    position: absolute; right: .6rem; top: 50%; transform: translateY(-50%);
    background: none; border: none; color: #4a6580; cursor: pointer; padding: .2rem;
    display: flex; align-items: center;
  }
  .toggle:hover { color: #94aabe; }
  .strength-row { display: flex; gap: .3rem; align-items: center; margin-top: .35rem; }
  .seg { height: 3px; flex: 1; border-radius: 99px; transition: background .2s; }
  .str-label { font-size: .72rem; font-weight: 600; margin-left: .2rem; min-width: 4rem; }
</style>
