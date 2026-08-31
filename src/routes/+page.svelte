<script lang="ts">
  import { activeVault, currentModal } from '$lib/stores/appState';
  import DesktopGrid from '$lib/components/DesktopGrid.svelte';
  import VaultExplorer from '$lib/components/VaultExplorer.svelte';
  import CreateVaultModal from '$lib/components/CreateVaultModal.svelte';
  import UnlockVaultModal from '$lib/components/UnlockVaultModal.svelte';
  import ExtraKeyModal from '$lib/components/ExtraKeyModal.svelte';
</script>

{#if $activeVault}
  <VaultExplorer />
{:else}
  <DesktopGrid />
{/if}

{#if $currentModal === 'create'}
  <CreateVaultModal />
{:else if $currentModal === 'unlock'}
  <UnlockVaultModal />
{:else if $currentModal === 'extra_key'}
  <ExtraKeyModal />
{/if}
