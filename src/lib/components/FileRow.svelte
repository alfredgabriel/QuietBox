<script lang="ts">
  export let name: string;
  export let size: number = 0;
  export let isDir: boolean = false;
  export let removable: boolean = false;
  export let selected: boolean = false;
  export let index: number = 0;

  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  function fmt(bytes: number): string {
    if (bytes === 0) return '—';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024**2) return `${(bytes/1024).toFixed(0)} KB`;
    if (bytes < 1024**3) return `${(bytes/1024**2).toFixed(1)} MB`;
    return `${(bytes/1024**3).toFixed(2)} GB`;
  }

  function ext(n: string): string {
    const parts = n.split('.');
    return parts.length > 1 ? parts.pop()!.toUpperCase() : '—';
  }
</script>

<div class="row" class:selected class:even={index%2===0}
     on:click on:keydown role="button" tabindex="0">
  <span class="icon">
    {#if isDir}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="#f59e0b" stroke="none"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
    {:else}
      <svg width="16" height="16" viewBox="0 0 24 24" fill="#38bdf8" stroke="none"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8" fill="#0b1120"/></svg>
    {/if}
  </span>
  <span class="name truncate">{name}</span>
  <span class="type text-mono">{isDir ? 'DIR' : ext(name)}</span>
  <span class="size text-mono">{fmt(size)}</span>
  {#if removable}
    <button class="rm" on:click|stopPropagation={() => dispatch('remove')}
      aria-label="Remove file">
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
    </button>
  {/if}
</div>

<style>
  .row {
    display: grid;
    grid-template-columns: 24px 1fr 60px 80px auto;
    align-items: center;
    gap: .6rem;
    padding: .45rem .75rem;
    cursor: default;
    border-radius: 4px;
    transition: background .12s;
  }
  .row.even { background: rgba(255,255,255,.018); }
  .row:hover { background: #162035; }
  .row.selected { background: rgba(56,189,248,.12); outline: 1px solid rgba(56,189,248,.25); }
  .icon { display: flex; align-items: center; }
  .name { font-size: .85rem; color: #f0f6ff; }
  .type { font-size: .72rem; color: #4a6580; }
  .size { font-size: .75rem; color: #94aabe; text-align: right; }
  .rm {
    background: none; border: none; color: #4a6580; cursor: pointer;
    display: flex; align-items: center; padding: .15rem;
    border-radius: 4px;
  }
  .rm:hover { color: #ef4444; background: rgba(239,68,68,.12); }
</style>
