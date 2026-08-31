<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { view, pendingFiles, openSession, isLoading, loadingProgress, loadingStatus } from '$lib/stores/appState';
  import PasswordDialog from '$lib/components/PasswordDialog.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let recent: any[] = [];
  let dragging = false;
  let showPwdDialog = false;
  let pendingOpenPath = '';

  onMount(async () => {
    recent = await invoke('get_recent_archives');
  });

  async function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragging = false;
    const items = Array.from(e.dataTransfer?.files || []).map((f: any) => f.path ?? f.name);
    // Check if it's a .qbv file
    if (items.length === 1 && items[0].endsWith('.qbv')) {
      pendingOpenPath = items[0];
      showPwdDialog = true;
      return;
    }
    // Otherwise, treat as files to encrypt
    pendingFiles.set(items);
    view.set('create');
  }

  async function onPickFiles() {
    const paths = await invoke<string[]>('pick_files');
    if (paths.length > 0) {
      pendingFiles.set(paths);
      view.set('create');
    }
  }

  async function onOpenArchive() {
    const path = await invoke<string | null>('pick_open_qbv');
    if (path) {
      pendingOpenPath = path;
      showPwdDialog = true;
    }
  }

  async function onOpenRecent(path: string) {
    pendingOpenPath = path;
    showPwdDialog = true;
  }

  async function handleUnlock(password: string) {
    showPwdDialog = false;
    isLoading.set(true);
    loadingProgress.set(0);
    loadingStatus.set('Decrypting…');
    try {
      const res = await invoke<any>('open_archive', {
        archivePath: pendingOpenPath,
        password
      });
      openSession.set({
        path: res.path,
        name: res.name,
        password,
        is_alt: res.is_alt,
        has_alt_key: res.has_alt_key,
        entries: res.entries
      });
      view.set('open');
    } catch {
      // pass error back — PasswordDialog will show again
      showPwdDialog = true;
    } finally {
      isLoading.set(false);
    }
  }

  async function removeRecent(path: string, e: MouseEvent) {
    e.stopPropagation();
    await invoke('remove_from_recent', { path });
    recent = recent.filter((r: any) => r.path !== path);
  }

  function fmtDate(secs: number) {
    const d = new Date(secs * 1000);
    const diff = (Date.now() - d.getTime()) / 1000;
    if (diff < 60) return 'just now';
    if (diff < 3600) return `${Math.floor(diff/60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff/3600)}h ago`;
    return d.toLocaleDateString();
  }
  function fmtSize(b: number) {
    if (b < 1024**2) return `${(b/1024).toFixed(0)} KB`;
    return `${(b/1024**2).toFixed(1)} MB`;
  }
</script>

<div class="home">
  <!-- Drop zone -->
  <div
    class="drop-zone" class:dragging
    role="button" tabindex="0"
    on:dragover|preventDefault={() => dragging = true}
    on:dragleave={() => dragging = false}
    on:drop={handleDrop}
    on:click={onPickFiles}
    on:keydown={(e) => e.key === 'Enter' && onPickFiles()}
  >
    <div class="dz-icon">
      <svg width="52" height="52" viewBox="0 0 24 24" fill="none" stroke={dragging ? '#38bdf8' : '#2a4a72'}
           stroke-width="1.4" stroke-linecap="round" stroke-linejoin="round">
        <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
        <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
        <line x1="12" y1="8" x2="12" y2="4"/>
        <polyline points="9 7 12 4 15 7"/>
      </svg>
    </div>
    <p class="dz-hint">{$t('drop_hint')}</p>
    <p class="dz-sub">{$t('drop_or_click')}</p>
  </div>

  <!-- Actions -->
  <div class="actions">
    <button class="btn btn-primary" on:click={() => view.set('create')}>
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><path d="M12 5v14M5 12h14"/></svg>
      {$t('new_archive')}
    </button>
    <button class="btn btn-ghost" on:click={onOpenArchive}>
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
      {$t('open_archive')}
    </button>
  </div>

  <!-- Recent archives -->
  {#if recent.length > 0}
    <div class="recent-section">
      <p class="section-label">{$t('recent')}</p>
      <div class="recent-list">
        {#each recent as r}
          <div class="recent-item" role="button" tabindex="0"
               on:click={() => onOpenRecent(r.path)}
               on:keydown={(e) => e.key === 'Enter' && onOpenRecent(r.path)}>
            <div class="ri-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#38bdf8" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
              </svg>
            </div>
            <div class="ri-info">
              <span class="ri-name truncate">{r.name}</span>
              <span class="ri-meta">{fmtSize(r.size_bytes)} · {fmtDate(r.opened_at)}</span>
            </div>
            {#if r.has_alt_key}
              <div class="alt-badge" title="Has alternative password">2</div>
            {/if}
            <button class="rm-recent" on:click={(e) => removeRecent(r.path, e)}
              aria-label="Remove from recent">
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            </button>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <p class="no-recent text-dim">{$t('no_recent')}</p>
  {/if}
</div>

{#if showPwdDialog}
  <PasswordDialog
    archiveName={pendingOpenPath.split(/[\\/]/).pop() || ''}
    on:unlock={(e) => handleUnlock(e.detail)}
    on:cancel={() => { showPwdDialog = false; }}
  />
{/if}

<style>
  .home {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    padding: 1.5rem;
    height: 100%;
    overflow-y: auto;
  }
  .drop-zone {
    border: 2px dashed #1e3050;
    border-radius: 14px;
    padding: 2.5rem 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: .6rem;
    cursor: pointer;
    transition: all .2s;
    background: #060b14;
    text-align: center;
  }
  .drop-zone:hover, .drop-zone.dragging {
    border-color: #38bdf8;
    background: rgba(56,189,248,.05);
  }
  .dz-icon { transition: transform .2s; }
  .drop-zone:hover .dz-icon { transform: translateY(-4px); }
  .dz-hint { font-size: 1rem; font-weight: 600; color: #94aabe; }
  .dz-sub  { font-size: .8rem; color: #4a6580; }
  .actions { display: flex; gap: .75rem; }
  .section-label { font-size: .78rem; font-weight: 600; color: #4a6580; text-transform: uppercase; letter-spacing: .06em; margin-bottom: .4rem; }
  .recent-section { display: flex; flex-direction: column; }
  .recent-list { display: flex; flex-direction: column; gap: .3rem; }
  .recent-item {
    display: grid;
    grid-template-columns: 36px 1fr auto auto;
    align-items: center;
    gap: .65rem;
    padding: .55rem .7rem;
    border-radius: 8px;
    cursor: pointer;
    background: #0b1120;
    border: 1px solid #1e3050;
    transition: all .15s;
  }
  .recent-item:hover { border-color: #2a4a72; background: #0f172a; }
  .ri-icon { display: flex; }
  .ri-info { display: flex; flex-direction: column; min-width: 0; }
  .ri-name { font-size: .88rem; font-weight: 500; color: #f0f6ff; }
  .ri-meta { font-size: .75rem; color: #4a6580; font-family: 'JetBrains Mono', monospace; }
  .alt-badge {
    width: 20px; height: 20px;
    background: rgba(245,158,11,.2);
    border: 1px solid rgba(245,158,11,.4);
    color: #f59e0b;
    border-radius: 50%;
    font-size: .72rem;
    font-weight: 700;
    display: flex; align-items: center; justify-content: center;
  }
  .rm-recent { background: none; border: none; color: #4a6580; cursor: pointer; padding: .2rem; border-radius: 4px; display: flex; }
  .rm-recent:hover { color: #ef4444; }
  .no-recent { font-size: .85rem; text-align: center; padding: 1rem 0; }
</style>
