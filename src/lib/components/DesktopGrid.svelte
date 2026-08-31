<script lang="ts">
  import { t } from '$lib/i18n/i18n';
  import { vaultsList, currentModal, selectedVaultId } from '$lib/stores/appState';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  const GRID_CELL_SIZE = 110; // width/height in px per desktop cell

  let draggingId: string | null = null;
  let dragStartX = 0;
  let dragStartY = 0;
  let itemInitialGX = 0;
  let itemInitialGY = 0;
  let currentDragPixelX = 0;
  let currentDragPixelY = 0;

  let contextMenu = {
    visible: false,
    x: 0,
    y: 0,
    targetVaultId: null as string | null
  };

  onMount(async () => {
    try {
      const items = await invoke<any[]>('list_vaults');
      vaultsList.set(items);
    } catch (e) {
      console.error(e);
    }
  });

  function handleDesktopContextMenu(e: MouseEvent) {
    e.preventDefault();
    contextMenu = {
      visible: true,
      x: e.clientX,
      y: e.clientY,
      targetVaultId: null
    };
  }

  function handleVaultContextMenu(e: MouseEvent, id: string) {
    e.preventDefault();
    e.stopPropagation();
    contextMenu = {
      visible: true,
      x: e.clientX,
      y: e.clientY,
      targetVaultId: id
    };
  }

  function closeContextMenu() {
    contextMenu.visible = false;
  }

  function openVault(id: string) {
    selectedVaultId.set(id);
    currentModal.set('unlock');
  }

  async function handleDeleteVault(id: string) {
    closeContextMenu();
    try {
      await invoke('delete_vault', { id });
      vaultsList.update(list => list.filter(v => v.id !== id));
    } catch (e) {
      console.error(e);
    }
  }

  function startDrag(e: MouseEvent, vault: any) {
    if (e.button !== 0) return; // Only left click
    draggingId = vault.id;
    dragStartX = e.clientX;
    dragStartY = e.clientY;
    itemInitialGX = vault.grid_x;
    itemInitialGY = vault.grid_y;
    currentDragPixelX = vault.grid_x * GRID_CELL_SIZE;
    currentDragPixelY = vault.grid_y * GRID_CELL_SIZE;

    window.addEventListener('mousemove', onDragging);
    window.addEventListener('mouseup', stopDrag);
  }

  function onDragging(e: MouseEvent) {
    if (!draggingId) return;
    const dx = e.clientX - dragStartX;
    const dy = e.clientY - dragStartY;
    currentDragPixelX = Math.max(0, itemInitialGX * GRID_CELL_SIZE + dx);
    currentDragPixelY = Math.max(0, itemInitialGY * GRID_CELL_SIZE + dy);
  }

  async function stopDrag() {
    if (!draggingId) return;
    const finalGX = Math.round(currentDragPixelX / GRID_CELL_SIZE);
    const finalGY = Math.round(currentDragPixelY / GRID_CELL_SIZE);

    vaultsList.update(list => list.map(v => {
      if (v.id === draggingId) {
        return { ...v, grid_x: finalGX, grid_y: finalGY };
      }
      return v;
    }));

    try {
      await invoke('update_vault_position', {
        id: draggingId,
        gridX: finalGX,
        gridY: finalGY
      });
    } catch (e) {
      console.error(e);
    }

    draggingId = null;
    window.removeEventListener('mousemove', onDragging);
    window.removeEventListener('mouseup', stopDrag);
  }
</script>

<svelte:window on:click={closeContextMenu} />

<div class="desktop-area" on:contextmenu={handleDesktopContextMenu}>
  <div class="desktop-grid-dots"></div>

  {#if $vaultsList.length === 0}
    <div class="empty-hint">
      <div class="empty-icon">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="#475569" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
      </div>
      <p>{$t('empty_desktop')}</p>
      <button class="btn-create" on:click={() => currentModal.set('create')}>
        {$t('new_vault')}
      </button>
    </div>
  {/if}

  {#each $vaultsList as vault (vault.id)}
    {@const isBeingDragged = draggingId === vault.id}
    {@const posX = isBeingDragged ? currentDragPixelX : vault.grid_x * GRID_CELL_SIZE}
    {@const posY = isBeingDragged ? currentDragPixelY : vault.grid_y * GRID_CELL_SIZE}

    <div
      class="vault-icon-item"
      class:dragging={isBeingDragged}
      style="transform: translate3d({posX}px, {posY}px, 0);"
      on:mousedown={(e) => startDrag(e, vault)}
      on:dblclick={() => openVault(vault.id)}
      on:contextmenu={(e) => handleVaultContextMenu(e, vault.id)}
    >
      <div class="icon-graphic">
        <svg width="44" height="44" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="11" width="18" height="11" rx="2" ry="2" fill="#0f172a" stroke="#38bdf8"></rect>
          <path d="M7 11V7a5 5 0 0 1 10 0v4" stroke="#38bdf8"></path>
          <circle cx="12" cy="16" r="1.5" fill="#38bdf8"></circle>
        </svg>
      </div>
      <span class="vault-label" title={vault.name}>{vault.name}</span>
    </div>
  {/each}
</div>

{#if contextMenu.visible}
  <div class="context-menu" style="top: {contextMenu.y}px; left: {contextMenu.x}px;">
    {#if contextMenu.targetVaultId}
      <button class="menu-item" on:click={() => openVault(contextMenu.targetVaultId!)}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"></path><path d="M10 14 21 3"></path><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path></svg>
        <span>{$t('unlock')}</span>
      </button>
      <button class="menu-item danger" on:click={() => handleDeleteVault(contextMenu.targetVaultId!)}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"></polyline><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path></svg>
        <span>{$t('delete')}</span>
      </button>
    {:else}
      <button class="menu-item" on:click={() => currentModal.set('create')}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14M5 12h14"></path></svg>
        <span>{$t('new_vault')}</span>
      </button>
      <button class="menu-item" on:click={() => vaultsList.update(list => [...list])}>
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.5 2v6h-6M21.34 15.57a10 10 0 1 1-.57-8.38l6-6.19"></path></svg>
        <span>{$t('refresh')}</span>
      </button>
    {/if}
  </div>
{/if}

<style>
  .desktop-area {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
    user-select: none;
  }
  .desktop-grid-dots {
    position: absolute;
    inset: 0;
    background-image: radial-gradient(rgba(51, 65, 85, 0.4) 1px, transparent 1px);
    background-size: 28px 28px;
    pointer-events: none;
  }
  .empty-hint {
    position: absolute;
    top: 40%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
    color: #64748b;
    font-size: 0.9rem;
    max-width: 320px;
  }
  .btn-create {
    background: #0284c7;
    border: none;
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-weight: 600;
    cursor: pointer;
    font-size: 0.85rem;
    margin-top: 0.5rem;
  }
  .vault-icon-item {
    position: absolute;
    top: 20px;
    left: 20px;
    width: 90px;
    height: 90px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
    border-radius: 0.5rem;
    cursor: pointer;
    padding: 0.5rem;
    transition: background 0.15s;
  }
  .vault-icon-item:hover {
    background: rgba(30, 41, 59, 0.6);
  }
  .vault-icon-item.dragging {
    z-index: 50;
    background: rgba(56, 189, 248, 0.15);
    border: 1px dashed #38bdf8;
    cursor: grabbing;
    pointer-events: none;
  }
  .icon-graphic {
    filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.4));
  }
  .vault-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: #f1f5f9;
    text-align: center;
    max-width: 80px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.8);
  }
  .context-menu {
    position: fixed;
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 0.5rem;
    padding: 0.35rem;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    z-index: 200;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    min-width: 150px;
  }
  .menu-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: none;
    border: none;
    color: #cbd5e1;
    padding: 0.45rem 0.65rem;
    border-radius: 0.25rem;
    font-size: 0.8rem;
    cursor: pointer;
    text-align: left;
    transition: background 0.1s;
  }
  .menu-item:hover {
    background: #1e293b;
    color: #f8fafc;
  }
  .menu-item.danger {
    color: #f87171;
  }
  .menu-item.danger:hover {
    background: rgba(239, 68, 68, 0.15);
  }
</style>
