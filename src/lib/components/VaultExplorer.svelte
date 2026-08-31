<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { activeVault, currentModal, isProcessing, processingStatus } from '$lib/stores/appState';
  import { invoke } from '@tauri-apps/api/core';

  function exitVault() {
    activeVault.set(null);
  }

  async function handleAddFiles() {
    if (!$activeVault) return;
    try {
      const res = await invoke<string | null>('pick_file');
      if (!res) return;

      isProcessing.set(true);
      processingStatus.set($t('importing'));

      const updatedEntries = await invoke<any[]>('import_files_to_vault', {
        id: $activeVault.id,
        password: $activeVault.password,
        filePaths: [res]
      });

      activeVault.update(v => v ? { ...v, entries: updatedEntries } : null);
    } catch (e) {
      console.error(e);
    } finally {
      isProcessing.set(false);
    }
  }

  async function handleExportAll() {
    if (!$activeVault) return;
    try {
      const dir = await invoke<string | null>('pick_directory');
      if (!dir) return;

      isProcessing.set(true);
      processingStatus.set($t('exporting'));

      await invoke('export_vault_to_dir', {
        id: $activeVault.id,
        password: $activeVault.password,
        outputDir: dir
      });
      alert($t('success_export'));
    } catch (e) {
      console.error(e);
    } finally {
      isProcessing.set(false);
    }
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<div class="explorer-container">
  <!-- Top Bar -->
  <div class="explorer-toolbar">
    <div class="left-group">
      <button class="btn back-btn" on:click={exitVault}>
        &larr; {$t('back_to_desktop')}
      </button>
      <div class="vault-info">
        <span class="vault-title">{$activeVault?.name}</span>
        <span class="badge">{$activeVault?.entries.length || 0} {$t('items')}</span>
      </div>
    </div>

    <div class="right-group">
      {#if !$activeVault?.is_extra}
        <button class="btn extra-key-btn" on:click={() => currentModal.set('extra_key')}>
          <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="7.5" cy="15.5" r="5.5"></circle><path d="m21 2-9.6 9.6"></path><path d="m15.5 7.5 3 3L22 7l-3-3"></path></svg>
          <span>{$t('extra_key')}</span>
        </button>
      {/if}
      <button class="btn primary" on:click={handleAddFiles}>
        {$t('add_files')}
      </button>
      <button class="btn secondary" on:click={handleExportAll}>
        {$t('export_all')}
      </button>
    </div>
  </div>

  <!-- Content List -->
  <div class="file-grid">
    {#if ($activeVault?.entries.length || 0) === 0}
      <div class="empty-state">
        <p>{$t('empty_vault')}</p>
        <button class="btn primary" on:click={handleAddFiles}>{$t('add_files')}</button>
      </div>
    {:else}
      {#each $activeVault?.entries || [] as entry}
        <div class="file-card">
          <div class="file-icon">
            {#if entry.is_dir}
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#f59e0b" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
            {:else}
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="#38bdf8" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline></svg>
            {/if}
          </div>
          <div class="file-details">
            <span class="file-name" title={entry.name}>{entry.name}</span>
            <span class="file-size">{formatSize(entry.size)}</span>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .explorer-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 1.25rem;
    background: #090d16;
  }
  .explorer-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid #1e293b;
  }
  .left-group, .right-group {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .vault-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .vault-title {
    font-size: 1.2rem;
    font-weight: 700;
    color: #f8fafc;
  }
  .badge {
    font-size: 0.75rem;
    background: #1e293b;
    color: #94a3b8;
    padding: 0.2rem 0.5rem;
    border-radius: 9999px;
  }
  .btn {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    padding: 0.45rem 0.85rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: all 0.15s;
  }
  .btn.back-btn {
    background: #1e293b;
    color: #cbd5e1;
  }
  .btn.back-btn:hover {
    background: #334155;
    color: #ffffff;
  }
  .btn.extra-key-btn {
    background: rgba(245, 158, 11, 0.15);
    color: #f59e0b;
    border: 1px solid rgba(245, 158, 11, 0.3);
  }
  .btn.extra-key-btn:hover {
    background: rgba(245, 158, 11, 0.25);
  }
  .btn.primary {
    background: #0284c7;
    color: #ffffff;
  }
  .btn.primary:hover {
    background: #0369a1;
  }
  .btn.secondary {
    background: #1e293b;
    color: #cbd5e1;
  }
  .file-grid {
    flex: 1;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(130px, 1fr));
    gap: 1rem;
    align-content: start;
    overflow-y: auto;
    padding: 0.5rem;
  }
  .empty-state {
    grid-column: 1 / -1;
    margin-top: 3rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    color: #64748b;
    font-size: 0.9rem;
  }
  .file-card {
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.5rem;
    padding: 0.85rem 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    text-align: center;
    transition: transform 0.15s, border-color 0.15s;
  }
  .file-card:hover {
    border-color: #38bdf8;
    transform: translateY(-2px);
  }
  .file-details {
    display: flex;
    flex-direction: column;
    width: 100%;
  }
  .file-name {
    font-size: 0.8rem;
    font-weight: 500;
    color: #f1f5f9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding: 0 0.2rem;
  }
  .file-size {
    font-size: 0.7rem;
    color: #64748b;
    font-family: 'JetBrains Mono', monospace;
  }
</style>
