<script lang="ts">
  import { view } from '$lib/stores/appState';
  import HomeScreen from '$lib/components/HomeScreen.svelte';
  import CreateView from '$lib/components/CreateView.svelte';
  import ArchiveView from '$lib/components/ArchiveView.svelte';
</script>

{#if $view === 'home'}
  <HomeScreen />
{:else if $view === 'create'}
  <CreateView />
{:else if $view === 'open'}
  <ArchiveView />
{/if}
