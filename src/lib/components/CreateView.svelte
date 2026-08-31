<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { view, pendingFiles, openSession, isLoading, loadingProgress, loadingStatus } from '$lib/stores/appState';
  import PasswordInput from '$lib/components/PasswordInput.svelte';
  import FileRow from '$lib/components/FileRow.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let files: string[] = [];
  let archiveName = '';
  let password = '';
  let altEnabled = false;
  let altPassword = '';
  let altFiles: string[] = [];
  let errorMsg = '';
  let savePath = '';

  // Initialise from pending files
  import { get } from 'svelte/store';
  files = get(pendingFiles);
  pendingFiles.set([]);

  async function pickMoreFiles() {
    const paths = await invoke<string[]>('pick_files');
    files = [...files, ...paths.filter(p => !files.includes(p))];
  }
  async function pickAltFiles() {
    const paths = await invoke<string[]>('pick_files');
    altFiles = [...altFiles, ...paths.filter(p => !altFiles.includes(p))];
  }

  async function handleCreate() {
    errorMsg = '';
    if (!archiveName.trim()) { errorMsg = $t('needs_name'); return; }
    if (files.length === 0)  { errorMsg = $t('needs_files'); return; }
    if (password.length < 4)  { errorMsg = $t('needs_password'); return; }
    if (altEnabled && altPassword.length < 4) { errorMsg = $t('needs_password'); return; }
    if (altEnabled && altPassword === password) { errorMsg = $t('alt_diff_from_primary'); return; }

    // Pick save location
    const outPath = await invoke<string | null>('pick_save_qbv', { defaultName: archiveName.trim() });
    if (!outPath) return;

    isLoading.set(true); loadingProgress.set(0); loadingStatus.set($t('creating'));
    try {
      await invoke('create_archive', {
        name: archiveName.trim(),
        files,
        password,
        outputPath: outPath,
      });

      if (altEnabled) {
        loadingStatus.set('Setting up alternative password…');
        await invoke('add_alt_password', {
          archivePath: outPath,
          altPassword,
          altFiles,
        });
      }

      // Open the just-created archive
      loadingStatus.set('Opening archive…');
      const res = await invoke<any>('open_archive', {
        archivePath: outPath,
        password,
      });
      openSession.set({
        path: res.path,
        name: res.name,
        password,
        is_alt: res.is_alt,
        has_alt_key: altEnabled,
        entries: res.entries,
      });
      view.set('open');
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : 'Failed to create archive.';
    } finally {
      isLoading.set(false);
    }
  }

  function removeFile(idx: number, alt = false) {
    if (alt) altFiles = altFiles.filter((_, i) => i !== idx);
    else     files     = files.filter((_, i) => i !== idx);
  }

  function basename(p: string) { return p.replace(/\\/g, '/').split('/').pop() || p; }

  function cancel() {
    pendingFiles.set([]);
    view.set('home');
  }
</script>

<div class="create-view">
  <div class="create-header">
    <button class="btn btn-ghost btn-sm" on:click={cancel}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><polyline points="15 18 9 12 15 6"/></svg>
      {$t('cancel')}
    </button>
    <h2>{$t('create_archive')}</h2>
  </div>

  <div class="create-body">
    <!-- Left column: files -->
    <div class="col">
      <div class="section">
        <span class="label">{$t('files_to_encrypt')}</span>
        <div class="file-box">
          {#each files as f, i}
            <FileRow name={basename(f)} removable index={i} on:remove={() => removeFile(i)}/>
          {/each}
          {#if files.length === 0}
            <p class="empty-hint text-dim">{$t('needs_files')}</p>
          {/if}
        </div>
        <button class="btn btn-ghost btn-sm add-btn" on:click={pickMoreFiles}>
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><path d="M12 5v14M5 12h14"/></svg>
          {$t('add_more_files')}
        </button>
      </div>
    </div>

    <!-- Right column: settings -->
    <div class="col">
      {#if errorMsg}
        <div class="err">{errorMsg}</div>
      {/if}

      <div class="section">
        <label class="label" for="arc-name">{$t('archive_name')}</label>
        <input id="arc-name" class="input" bind:value={archiveName}
               placeholder={$t('archive_name_ph')} />
      </div>

      <PasswordInput label={$t('primary_password')} bind:value={password}/>

      <!-- Alt password toggle -->
      <div class="alt-toggle">
        <label class="toggle-label">
          <input type="checkbox" bind:checked={altEnabled} />
          <span class="toggle-track"><span class="toggle-thumb"></span></span>
          <span>{$t('enable_alt')}</span>
        </label>
      </div>

      {#if altEnabled}
        <div class="alt-section">
          <p class="alt-desc text-dim">{$t('alt_password_desc')}</p>
          <PasswordInput label={$t('alt_password')} bind:value={altPassword}/>

          <div class="section">
            <span class="label">{$t('alt_files')}</span>
            <div class="file-box small">
              {#each altFiles as f, i}
                <FileRow name={basename(f)} removable index={i} on:remove={() => removeFile(i, true)}/>
              {/each}
              {#if altFiles.length === 0}
                <p class="empty-hint text-dim text-sm">{$t('add_alt_files')}</p>
              {/if}
            </div>
            <button class="btn btn-ghost btn-sm add-btn" on:click={pickAltFiles}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round"><path d="M12 5v14M5 12h14"/></svg>
              {$t('add_alt_files')}
            </button>
          </div>
        </div>
      {/if}

      <button class="btn btn-primary create-btn" on:click={handleCreate}>
        <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v14z"/><polyline points="17 21 17 13 7 13"/><polyline points="7 3 7 8 15 8"/></svg>
        {$t('create')}
      </button>
    </div>
  </div>
</div>

<style>
  .create-view {
    display: flex; flex-direction: column; height: 100%;
    background: #060b14;
  }
  .create-header {
    display: flex; align-items: center; gap: 1rem;
    padding: .85rem 1.25rem;
    border-bottom: 1px solid #1e3050;
    background: #0b1120;
  }
  h2 { font-size: 1rem; font-weight: 700; color: #f0f6ff; }
  .create-body {
    display: grid; grid-template-columns: 1fr 380px;
    gap: 0; flex: 1; overflow: hidden;
  }
  .col {
    padding: 1.25rem;
    display: flex; flex-direction: column; gap: .9rem;
    overflow-y: auto;
  }
  .col:first-child { border-right: 1px solid #1e3050; }
  .file-box {
    background: #060b14;
    border: 1px solid #1e3050;
    border-radius: 8px;
    min-height: 80px;
    overflow-y: auto;
    max-height: 280px;
    padding: .3rem;
  }
  .file-box.small { max-height: 160px; }
  .empty-hint { padding: 1rem; text-align: center; font-size: .82rem; }
  .add-btn { align-self: flex-start; margin-top: .3rem; }
  .err {
    background: rgba(239,68,68,.12);
    border: 1px solid rgba(239,68,68,.3);
    color: #f87171;
    padding: .5rem .75rem;
    border-radius: 6px;
    font-size: .82rem;
  }
  .alt-toggle { display: flex; align-items: center; }
  .toggle-label {
    display: flex; align-items: center; gap: .6rem;
    cursor: pointer; font-size: .88rem; color: #94aabe;
    user-select: none;
  }
  .toggle-label input { display: none; }
  .toggle-track {
    width: 36px; height: 20px;
    background: #1e3050; border-radius: 99px;
    position: relative; transition: background .2s;
    flex-shrink: 0;
  }
  .toggle-label input:checked ~ .toggle-track { background: #0ea5e9; }
  .toggle-thumb {
    position: absolute; top: 2px; left: 2px;
    width: 16px; height: 16px;
    background: #fff; border-radius: 50%;
    transition: left .2s;
  }
  .toggle-label input:checked ~ .toggle-track .toggle-thumb { left: 18px; }
  .alt-section {
    background: rgba(245,158,11,.05);
    border: 1px solid rgba(245,158,11,.2);
    border-radius: 10px;
    padding: 1rem;
    display: flex; flex-direction: column; gap: .85rem;
  }
  .alt-desc { font-size: .8rem; line-height: 1.5; }
  .create-btn { align-self: flex-end; padding: .6rem 1.5rem; }
  .text-sm { font-size: .8rem; }
</style>
