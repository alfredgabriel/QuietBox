<script lang="ts">
  import '../app.css';
  import Navbar from '$lib/components/Navbar.svelte';
</script>

<div class="app-shell">
  <Navbar />
  <main class="viewport">
    <slot />
  </main>
</div>

<style>
  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    background: #090d16;
  }
  .viewport {
    flex: 1;
    overflow: hidden;
    position: relative;
  }
</style>
