<script lang="ts">
  import { t } from '$lib/i18n/i18n';

  export let value = '';
  export let label = '';
  export let placeholder = '••••••••••••••••';
  export let showStrength = true;

  let showPassword = false;

  function calculateStrength(pwd: string): number {
    if (!pwd) return 0;
    let score = 0;
    if (pwd.length >= 8) score += 1;
    if (pwd.length >= 14) score += 1;
    if (/[A-Z]/.test(pwd) && /[a-z]/.test(pwd)) score += 1;
    if (/[0-9]/.test(pwd) && /[^A-Za-z0-9]/.test(pwd)) score += 1;
    return score;
  }

  $: strength = calculateStrength(value);
  $: strengthColor =
    strength === 0 ? '#374151' :
    strength === 1 ? '#ef4444' :
    strength === 2 ? '#f59e0b' :
    strength === 3 ? '#3b82f6' : '#10b981';

  $: strengthLabel =
    strength === 0 ? '' :
    strength === 1 ? $t('password_weak') :
    strength === 2 ? $t('password_fair') :
    strength === 3 ? $t('password_good') : $t('password_strong');
</script>

<div class="pwd-field">
  {#if label}
    <label class="pwd-label">{label}</label>
  {/if}
  <div class="input-wrapper">
    <input
      type={showPassword ? 'text' : 'password'}
      bind:value
      {placeholder}
      class="pwd-input"
    />
    <button
      type="button"
      class="toggle-btn"
      on:click={() => (showPassword = !showPassword)}
      title="Show/Hide"
    >
      {#if showPassword}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>
      {:else}
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
      {/if}
    </button>
  </div>

  {#if showStrength && value}
    <div class="strength-meter">
      <div class="bars">
        {#each [1, 2, 3, 4] as step}
          <div
            class="bar"
            style="background-color: {strength >= step ? strengthColor : '#1f2937'};"
          ></div>
        {/each}
      </div>
      <span class="strength-text" style="color: {strengthColor};">{strengthLabel}</span>
    </div>
  {/if}
</div>

<style>
  .pwd-field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    width: 100%;
  }

  .pwd-label {
    font-size: 0.85rem;
    font-weight: 500;
    color: #94a3b8;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .pwd-input {
    width: 100%;
    background: #0f172a;
    border: 1px solid #334155;
    color: #f8fafc;
    padding: 0.65rem 2.5rem 0.65rem 0.85rem;
    border-radius: 0.5rem;
    font-size: 0.95rem;
    font-family: 'JetBrains Mono', monospace;
    outline: none;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .pwd-input:focus {
    border-color: #38bdf8;
    box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.2);
  }

  .toggle-btn {
    position: absolute;
    right: 0.75rem;
    background: none;
    border: none;
    color: #64748b;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    transition: color 0.15s;
  }

  .toggle-btn:hover {
    color: #cbd5e1;
  }

  .strength-meter {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .bars {
    display: flex;
    gap: 0.25rem;
    flex: 1;
  }

  .bar {
    height: 4px;
    flex: 1;
    border-radius: 2px;
    transition: background-color 0.25s ease;
  }

  .strength-text {
    font-size: 0.75rem;
    font-weight: 600;
  }
</style>
