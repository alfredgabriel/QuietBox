<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { invoke } from '@tauri-apps/api/core';

  export let files: string[] = [];
  export let title: string = '';

  async function handleBrowseFile() {
    try {
      const res = await invoke<string | null>('pick_file');
      if (res && !files.includes(res)) {
        files = [...files, res];
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function handleBrowseFolder() {
    try {
      const res = await invoke<string | null>('pick_directory');
      if (res && !files.includes(res)) {
        files = [...files, res];
      }
    } catch (e) {
      console.error(e);
    }
  }

  function removeFile(index: number) {
    files = files.filter((_, i) => i !== index);
  }

  function clearAll() {
    files = [];
  }
</script>

<div class="drop-zone-container">
  {#if title}
    <div class="header">
      <span class="title">{title}</span>
      {#if files.length > 0}
        <button class="clear-btn" on:click={clearAll}>{$t('clear_selection')}</button>
      {/if}
    </div>
  {/if}

  <div class="drop-box">
    <div class="actions">
      <button type="button" class="action-btn" on:click={handleBrowseFile}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="12" y1="18" x2="12" y2="12"></line><line x1="9" y1="15" x2="15" y2="15"></line></svg>
        <span>+ Add Files</span>
      </button>
      <button type="button" class="action-btn secondary" on:click={handleBrowseFolder}>
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="12" y1="11" x2="12" y2="17"></line><line x1="9" y1="14" x2="15" y2="14"></line></svg>
        <span>+ Add Folder</span>
      </button>
    </div>
  </div>

  {#if files.length > 0}
    <div class="file-list">
      <div class="list-summary">
        <span>{files.length} {$t('files_selected')}</span>
      </div>
      {#each files as file, idx}
        <div class="file-item">
          <span class="file-path" title={file}>{file}</span>
          <button class="remove-btn" on:click={() => removeFile(idx)}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .drop-zone-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .title {
    font-size: 0.85rem;
    font-weight: 500;
    color: #94a3b8;
  }

  .clear-btn {
    background: none;
    border: none;
    color: #ef4444;
    font-size: 0.75rem;
    cursor: pointer;
  }

  .drop-box {
    border: 1px dashed #334155;
    background: rgba(15, 23, 42, 0.5);
    border-radius: 0.5rem;
    padding: 1rem;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: border-color 0.2s;
  }

  .drop-box:hover {
    border-color: #475569;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    background: #1e293b;
    border: 1px solid #334155;
    color: #f1f5f9;
    padding: 0.45rem 0.85rem;
    border-radius: 0.375rem;
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.15s;
  }

  .action-btn:hover {
    background: #334155;
    border-color: #64748b;
  }

  .action-btn.secondary {
    background: #0f172a;
  }

  .file-list {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    max-height: 120px;
    overflow-y: auto;
    background: #0f172a;
    border: 1px solid #1e293b;
    border-radius: 0.375rem;
    padding: 0.5rem;
  }

  .list-summary {
    font-size: 0.75rem;
    color: #64748b;
    font-weight: 600;
    margin-bottom: 0.2rem;
  }

  .file-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: #1e293b;
    padding: 0.3rem 0.5rem;
    border-radius: 0.25rem;
    font-size: 0.8rem;
  }

  .file-path {
    color: #cbd5e1;
    font-family: 'JetBrains Mono', monospace;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 85%;
  }

  .remove-btn {
    background: none;
    border: none;
    color: #94a3b8;
    cursor: pointer;
    display: flex;
    align-items: center;
    padding: 2px;
  }

  .remove-btn:hover {
    color: #ef4444;
  }
</style>
