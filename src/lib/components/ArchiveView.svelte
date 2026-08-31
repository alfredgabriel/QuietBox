<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { view, openSession, isLoading, loadingProgress, loadingStatus } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import FileRow from '$lib/components/FileRow.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let session = $openSession!;
  let selected = new Set<number>();
  let showAltSetup = false;
  let altPassword = '';
  let altFiles: string[] = [];
  let altError = '';

  $: totalSize = session.entries.reduce((s, e) => s + e.size, 0);
  $: totalFiles = session.entries.filter(e => !e.is_dir).length;

  function fmtSize(b: number) {
    if (b < 1024**2) return `${(b/1024).toFixed(0)} KB`;
    if (b < 1024**3) return `${(b/1024**2).toFixed(1)} MB`;
    return `${(b/1024**3).toFixed(2)} GB`;
  }

  function toggleSelect(i: number) {
    const s = new Set(selected);
    s.has(i) ? s.delete(i) : s.add(i);
    selected = s;
  }



  async function deleteSelected() {
    if (selected.size === 0) return;
    if (!confirm($t('confirm_delete'))) return;

    const paths = [...selected].map(i => session.entries[i].path);
    isLoading.set(true); loadingStatus.set($t('deleting')); loadingProgress.set(0);
    try {
      const entries = await invoke<any[]>('delete_from_archive', {
        archivePath: session.path,
        password: session.password,
        entryPaths: paths,
      });
      openSession.update(s => s ? { ...s, entries } : s);
      session = { ...session, entries };
      selected = new Set();
    } finally { isLoading.set(false); }
  }

  async function extractSelected() {
    if (selected.size === 0) return;
    const dir = await invoke<string | null>('pick_directory');
    if (!dir) return;
    const paths = [...selected].map(i => session.entries[i].path);
    isLoading.set(true); loadingStatus.set($t('extracting')); loadingProgress.set(0);
    try {
      await invoke('extract_files', {
        archivePath: session.path,
        password: session.password,
        entryPaths: paths,
        outputDir: dir,
      });
    } finally { isLoading.set(false); }
  }

  async function addFiles() {
    const paths = await invoke<string[]>('pick_files');
    if (!paths.length) return;
    isLoading.set(true); loadingStatus.set($t('adding')); loadingProgress.set(0);
    try {
      const entries = await invoke<any[]>('add_to_archive', {
        archivePath: session.path,
        password: session.password,
        newFiles: paths,
      });
      openSession.update(s => s ? { ...s, entries } : s);
      session = { ...session, entries };
    } finally { isLoading.set(false); }
  }

  async function extractAll() {
    const dir = await invoke<string | null>('pick_directory');
    if (!dir) return;
    isLoading.set(true); loadingStatus.set($t('extracting')); loadingProgress.set(0);
    try {
      await invoke('extract_archive', {
        archivePath: session.path,
        password: session.password,
        outputDir: dir,
      });
    } finally { isLoading.set(false); }
  }

  async function setupAlt() {
    if (altPassword.length < 4) { altError = $t('needs_password'); return; }
    if (altPassword === session.password) { altError = $t('alt_diff_from_primary'); return; }
    altError = '';
    isLoading.set(true); loadingStatus.set('Setting up alternative password…'); loadingProgress.set(0);
    try {
      await invoke('add_alt_password', {
        archivePath: session.path,
        altPassword,
        altFiles,
      });
      openSession.update(s => s ? { ...s, has_alt_key: true } : s);
      session = { ...session, has_alt_key: true };
      showAltSetup = false;
    } catch (e: any) {
      altError = typeof e === 'string' ? e : 'Failed.';
    } finally { isLoading.set(false); }
  }

  async function pickAltFiles() {
    const paths = await invoke<string[]>('pick_files');
    altFiles = [...altFiles, ...paths.filter(p => !altFiles.includes(p))];
  }

  function basename(p: string) { return p.replace(/\\/g, '/').split('/').pop() || p; }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const paths = Array.from(e.dataTransfer?.files || []).map((f: any) => f.path ?? f.name);
    if (paths.length) {
      // add files
      isLoading.set(true); loadingStatus.set($t('adding')); loadingProgress.set(0);
      invoke<any[]>('add_to_archive', {
        archivePath: session.path,
        password: session.password,
        newFiles: paths,
      }).then(entries => {
        openSession.update(s => s ? { ...s, entries } : s);
        session = { ...session, entries };
      }).finally(() => isLoading.set(false));
    }
  }
</script>

<div class="archive-view"
     on:dragover|preventDefault
     on:drop={handleDrop}>

  <!-- Title bar -->
  <div class="av-header">
    <div class="av-title">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#38bdf8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
        <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
      </svg>
      <span class="av-name">{session.name}.qbv</span>
      {#if session.is_alt}
        <span class="badge amber">{$t('opened_alt')}</span>
      {:else}
        <span class="badge cyan">{$t('opened_primary')}</span>
      {/if}
    </div>
    <div class="av-actions">
      {#if !session.has_alt_key && !session.is_alt}
        <button class="btn btn-ghost btn-sm" on:click={() => showAltSetup = true}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="7.5" cy="15.5" r="5.5"/><path d="m21 2-9.6 9.6"/><path d="m15.5 7.5 3 3L22 7l-3-3"/></svg>
          {$t('setup_alt_key')}
        </button>
      {/if}
      <button class="btn btn-ghost btn-sm" on:click={() => { openSession.set(null); view.set('home'); }}>
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg>
        {$t('close_archive')}
      </button>
    </div>
  </div>

  <!-- Toolbar -->
  <div class="toolbar">
    <button class="tool-btn" on:click={addFiles}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"/></svg>
      <span>{$t('add_files')}</span>
    </button>
    {#if selected.size > 0}
    <button class="tool-btn danger" on:click={deleteSelected}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
      <span>{$t('delete_selected')} ({selected.size})</span>
    </button>
    {/if}
    {#if selected.size > 0}
    <button class="tool-btn accent" on:click={extractSelected}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
      <span>{$t('extract_selected')} ({selected.size})</span>
    </button>
    {/if}
    <button class="tool-btn" on:click={extractAll}>
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
      <span>{$t('extract_all')}</span>
    </button>
    <div class="sep"></div>
    <span class="path-chip">{session.path.replace(/\\/g, '/')}</span>
  </div>

  <!-- Column headers -->
  <div class="col-header">
    <span></span>
    <span>{$t('file_name')}</span>
    <span>{$t('file_type')}</span>
    <span class="right">{$t('file_size')}</span>
    <span></span>
  </div>

  <!-- File list -->
  <div class="file-list">
    {#if session.entries.length === 0}
      <div class="empty-state">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="#2a4a72" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>
        <p class="text-dim">{$t('add_first_files')}</p>
        <button class="btn btn-primary btn-sm" on:click={addFiles}>{$t('add_files')}</button>
      </div>
    {:else}
      {#each session.entries as entry, i}
        <FileRow
          name={entry.name}
          size={entry.size}
          isDir={entry.is_dir}
          index={i}
          selected={selected.has(i)}
          on:click={() => toggleSelect(i)}
        />
      {/each}
    {/if}
  </div>

  <!-- Status bar -->
  <div class="status-bar">
    <span>{totalFiles} {$t('total_files')} · {fmtSize(totalSize)} {$t('total_size')}</span>
    {#if selected.size > 0}
      <span class="selected-count">{selected.size} selected</span>
    {/if}
  </div>
</div>

<!-- Alt password setup panel -->
{#if showAltSetup}
  <div class="backdrop" role="dialog" aria-modal="true">
    <div class="alt-dlg">
      <h3>{$t('setup_alt_key')}</h3>
      <p class="alt-desc text-dim">{$t('alt_password_desc')}</p>
      {#if altError}
        <div class="err">{altError}</div>
      {/if}
      <PasswordInput label={$t('alt_password')} bind:value={altPassword}/>
      <div class="section">
        <span class="label">{$t('alt_files')} <span class="text-dim">(optional)</span></span>
        {#each altFiles as f, i}
          <div class="mini-file">
            <span class="truncate">{basename(f)}</span>
            <button class="rm-btn" on:click={() => altFiles = altFiles.filter((_,j)=>j!==i)} aria-label="Remove">×</button>
          </div>
        {/each}
        <button class="btn btn-ghost btn-sm" on:click={pickAltFiles}>{$t('add_alt_files')}</button>
      </div>
      <div class="dlg-foot">
        <button class="btn btn-ghost" on:click={() => showAltSetup = false}>{$t('cancel')}</button>
        <button class="btn btn-primary" on:click={setupAlt}>{$t('save_key') ?? 'Save'}</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .archive-view { display: flex; flex-direction: column; height: 100%; background: #060b14; }
  .av-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: .65rem 1rem;
    background: #0b1120;
    border-bottom: 1px solid #1e3050;
  }
  .av-title { display: flex; align-items: center; gap: .6rem; }
  .av-name { font-size: .9rem; font-weight: 600; color: #f0f6ff; font-family: 'JetBrains Mono', monospace; }
  .badge {
    font-size: .68rem; font-weight: 600; padding: .15rem .45rem;
    border-radius: 99px; letter-spacing: .03em;
  }
  .badge.cyan   { background: rgba(56,189,248,.15); color: #38bdf8; border: 1px solid rgba(56,189,248,.3); }
  .badge.amber  { background: rgba(245,158,11,.15); color: #f59e0b; border: 1px solid rgba(245,158,11,.3); }
  .av-actions { display: flex; gap: .5rem; }

  .toolbar {
    display: flex; align-items: center; gap: .35rem;
    padding: .5rem .75rem;
    background: #0b1120;
    border-bottom: 1px solid #1e3050;
  }
  .tool-btn {
    display: flex; align-items: center; gap: .35rem;
    background: none; border: 1px solid transparent;
    color: #94aabe; padding: .35rem .65rem;
    border-radius: 6px; cursor: pointer; font-size: .82rem; font-weight: 500;
    transition: all .15s;
  }
  .tool-btn:hover { background: #162035; border-color: #2a4a72; color: #f0f6ff; }
  .tool-btn.accent { color: #38bdf8; border-color: rgba(56,189,248,.3); }
  .tool-btn.accent:hover { background: rgba(56,189,248,.1); border-color: #38bdf8; }
  .tool-btn.danger { color: #f87171; border-color: rgba(239,68,68,.3); }
  .tool-btn.danger:hover { background: rgba(239,68,68,.12); border-color: #ef4444; }
  .sep { width: 1px; height: 20px; background: #1e3050; margin: 0 .25rem; }
  .path-chip { font-family: 'JetBrains Mono', monospace; font-size: .72rem; color: #4a6580; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; flex: 1; }

  .col-header {
    display: grid; grid-template-columns: 24px 1fr 60px 80px 30px;
    gap: .6rem; padding: .35rem .75rem;
    font-size: .72rem; font-weight: 600;
    color: #4a6580; text-transform: uppercase; letter-spacing: .05em;
    border-bottom: 1px solid #1e3050;
    background: #0b1120;
  }
  .right { text-align: right; }
  .file-list { flex: 1; overflow-y: auto; padding: .3rem .5rem; }
  .empty-state { display: flex; flex-direction: column; align-items: center; gap: .75rem; padding: 3rem 1rem; }
  .status-bar {
    display: flex; align-items: center; justify-content: space-between;
    padding: .35rem .85rem;
    font-size: .75rem; color: #4a6580;
    border-top: 1px solid #1e3050;
    background: #0b1120;
    font-family: 'JetBrains Mono', monospace;
  }
  .selected-count { color: #38bdf8; }

  .backdrop {
    position: fixed; inset: 0;
    background: rgba(6,11,20,.8); backdrop-filter: blur(6px);
    display: flex; align-items: center; justify-content: center;
    z-index: 500;
  }
  .alt-dlg {
    background: #0f172a; border: 1px solid #1e3050;
    border-radius: 14px; padding: 1.5rem;
    width: 400px; display: flex; flex-direction: column; gap: 1rem;
    box-shadow: 0 24px 64px rgba(0,0,0,.7);
  }
  .alt-dlg h3 { font-size: 1.05rem; font-weight: 700; color: #f0f6ff; }
  .alt-desc { font-size: .82rem; line-height: 1.5; }
  .err { background: rgba(239,68,68,.12); border: 1px solid rgba(239,68,68,.3); color: #f87171; padding: .5rem .75rem; border-radius: 6px; font-size: .82rem; }
  .mini-file { display: flex; align-items: center; justify-content: space-between; padding: .3rem .5rem; background: #060b14; border-radius: 4px; font-size: .8rem; color: #94aabe; }
  .rm-btn { background: none; border: none; color: #4a6580; cursor: pointer; font-size: 1rem; padding: 0 .2rem; }
  .rm-btn:hover { color: #ef4444; }
  .dlg-foot { display: flex; justify-content: flex-end; gap: .6rem; }
</style>
