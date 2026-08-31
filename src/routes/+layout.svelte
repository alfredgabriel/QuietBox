<script lang="ts">
  import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte';
  import { panicWipe } from '$lib/stores/appState';
  import { onMount } from 'svelte';

  onMount(() => {
    function handleKeydown(e: KeyboardEvent) {
      // Global panic hotkey: Ctrl + Shift + Q
      if (e.ctrlKey && e.shiftKey && (e.key === 'Q' || e.key === 'q')) {
        e.preventDefault();
        panicWipe();
      }
    }
    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="app-layout">
  <Navbar />
  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  .app-layout {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
    display: flex;
    justify-content: center;
  }
</style>
