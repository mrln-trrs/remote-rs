<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { fade, fly, slide } from 'svelte/transition'

  import PageTabs from './lib/PageTabs.svelte'
  import MiniPlayer from './lib/MiniPlayer.svelte'
  
  import HomePage from './lib/pages/HomePage.svelte'
  import PlayerPage from './lib/pages/PlayerPage.svelte'
  import ControlsPage from './lib/pages/ControlsPage.svelte'
  import SystemPage from './lib/pages/SystemPage.svelte'

  import { pageTabs } from './lib/data/pages'
  import { currentPath, initRouter } from './lib/stores/router'
  import { startMediaRealtimeSync } from './lib/stores/media'

  let syncCleanup: () => void

  onMount(() => {
    initRouter()
    syncCleanup = startMediaRealtimeSync()
  })

  onDestroy(() => {
    if (syncCleanup) syncCleanup()
  })
</script>

<div class="app-shell">
  <!-- Content Area -->
  <main class="content-viewport">
    {#key $currentPath}
      <div 
        class="page-wrapper"
        in:fade={{ duration: 200, delay: 150 }}
        out:fade={{ duration: 150 }}
      >
        {#if $currentPath === '/'}
          <HomePage />
        {:else if $currentPath === '/controls'}
          <ControlsPage />
        {:else if $currentPath === '/system'}
          <SystemPage />
        {:else if $currentPath === '/player'}
           <div class="player-v-spacer"></div>
        {/if}
      </div>
    {/key}
  </main>

  <!-- Immersive Player Overlay (Slides up from bottom) -->
  {#if $currentPath === '/player'}
    <div 
      class="player-overlay"
      transition:fly={{ y: 800, duration: 450, opacity: 1 }}
    >
      <PlayerPage />
    </div>
  {/if}

  <!-- Bottom Layers -->
  <footer class="bottom-shell">
    {#if $currentPath !== '/player'}
      <div transition:slide={{ axis: 'y' }}>
        <MiniPlayer />
      </div>
    {/if}
    <PageTabs items={pageTabs} activePath={$currentPath} />
  </footer>
</div>

<style>
  :global(:root) {
    --bg: #030303;
    --surface: #121217;
    --accent: #45d7ff;
    --accent-2: #6df0bf;
    --text: rgba(255, 255, 255, 0.7);
    --text-strong: #ffffff;
    --border: rgba(255, 255, 255, 0.05);
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--bg);
    color: var(--text);
    font-family: 'Roboto', 'Inter', sans-serif;
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
  }

  .app-shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    height: 100dvh;
    width: 100vw;
    position: relative;
    overflow: hidden;
  }

  .content-viewport {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    scrollbar-width: none;
    position: relative;
  }

  .page-wrapper {
    padding: 16px;
    padding-bottom: 24px;
    width: 100%;
    box-sizing: border-box;
  }

  .player-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 200;
  }

  .bottom-shell {
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    z-index: 100;
  }

  .player-v-spacer {
      height: 80vh;
  }

  :global(.surface) {
    background-color: var(--surface);
    border: 1px solid var(--border);
  }
</style>
