<script lang="ts">
  import { Play, Pause, SkipForward } from '@lucide/svelte'
  import { media, sendMediaCommand } from './stores/media'
  import { goTo } from './stores/router'

  let artworkFailed = false
  $: artworkSource = $media.artworkUrl
  $: if (artworkSource) artworkFailed = false
</script>

<div class="mini-player" onclick={() => goTo('/player')}>
  <div class="art-container">
    {#if artworkSource && !artworkFailed}
      <img src={artworkSource} alt="Art" onerror={() => (artworkFailed = true)} />
    {:else}
      <div class="placeholder">🎵</div>
    {/if}
  </div>

  <div class="info">
    <span class="title">{$media.title}</span>
    <span class="artist">{$media.artist}</span>
  </div>

  <div class="actions">
    <button onclick={(e) => { e.stopPropagation(); sendMediaCommand('playpause'); }}>
      {#if $media.isPlaying}
        <Pause size={28} fill="white" />
      {:else}
        <Play size={28} fill="white" />
      {/if}
    </button>
    <button onclick={(e) => { e.stopPropagation(); sendMediaCommand('next'); }}>
      <SkipForward size={24} fill="white" />
    </button>
  </div>
  
  <div class="progress-bar" style={`width: ${($media.progress / ($media.duration || 1)) * 100}%`}></div>
</div>

<style>
  .mini-player {
    display: flex;
    align-items: center;
    gap: 12px;
    height: 64px;
    background: #1b1b22;
    padding: 0 12px;
    position: relative;
    overflow: hidden;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  }

  .art-container {
    width: 44px;
    height: 44px;
    border-radius: 4px;
    overflow: hidden;
    background: #333;
    flex-shrink: 0;
  }

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .placeholder {
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
    font-size: 1.2rem;
  }

  .info {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    min-width: 0;
  }

  .title {
    color: white;
    font-size: 0.9rem;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .artist {
    color: rgba(255, 255, 255, 0.5);
    font-size: 0.8rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  button {
    background: none;
    border: none;
    color: white;
    padding: 8px;
    cursor: pointer;
  }

  .progress-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    height: 2px;
    background: white;
    transition: width 0.3s linear;
  }
</style>
