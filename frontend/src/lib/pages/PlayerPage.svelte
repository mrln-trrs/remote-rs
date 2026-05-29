<script lang="ts">
  import { ChevronDown, MoreVertical, Share2, ListMusic } from '@lucide/svelte'
  import { media, sendMediaCommand } from '../stores/media'
  import MediaControl from '../MediaControl.svelte'
  import { goTo } from '../stores/router'

  let artworkFailed = false
  $: artworkSource = $media.artworkUrl
  $: if (artworkSource) artworkFailed = false

  function formatTime(ms: number) {
    if (!ms || ms < 0) return '0:00'
    const totalSeconds = Math.floor(ms / 1000)
    const minutes = Math.floor(totalSeconds / 60)
    const seconds = totalSeconds % 60
    return `${minutes}:${seconds.toString().padStart(2, '0')}`
  }
</script>

<div class="immersive-player">
  <!-- Blurred Background -->
  <div class="background-blur">
    {#if artworkSource && !artworkFailed}
      <img src={artworkSource} alt="" />
    {/if}
  </div>

  <header class="player-header">
    <button class="icon-btn" onclick={() => goTo('/')}>
      <ChevronDown size={28} />
    </button>
    <div class="header-center">
      <span class="eyebrow">Reproduciendo ahora</span>
    </div>
    <button class="icon-btn">
      <MoreVertical size={24} />
    </button>
  </header>

  <main class="player-content">
    <div class="artwork-wrapper">
      <div class="artwork-shadow">
        {#if artworkSource && !artworkFailed}
          <img src={artworkSource} alt="Art" class="main-art" onerror={() => (artworkFailed = true)} />
        {:else}
          <div class="placeholder-art">🎵</div>
        {/if}
      </div>
    </div>

    <div class="meta-section">
      <div class="title-row">
        <div class="text-meta">
          <h1>{$media.title}</h1>
          <h2>{$media.artist}</h2>
        </div>
        <button class="icon-btn heart">
          <Share2 size={24} />
        </button>
      </div>

      <!-- Real-time Progress Section -->
      <div class="progress-container">
        <div class="progress-bar-bg">
          <div class="progress-bar-fill" style={`width: ${($media.progress / ($media.duration || 1)) * 100}%`}></div>
        </div>
        <div class="time-labels">
          <span>{formatTime($media.progress)}</span>
          <span>{formatTime($media.duration)}</span>
        </div>
      </div>
    </div>

    <div class="controls-section">
      <MediaControl />
    </div>
  </main>

  <footer class="player-footer">
    <button class="footer-action">
      <ListMusic size={20} />
      <span>Siguiente</span>
    </button>
  </footer>
</div>

<style>
  .immersive-player {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    height: 100dvh;
    background: #000;
    display: flex;
    flex-direction: column;
    z-index: 200;
    overflow: hidden;
    color: white;
  }

  .background-blur {
    position: absolute;
    top: -10%;
    left: -10%;
    width: 120%;
    height: 120%;
    z-index: -1;
    opacity: 0.45;
    filter: blur(80px) saturate(1.5);
  }

  .background-blur img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .player-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
  }

  .header-center {
    text-align: center;
  }

  .player-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 0 24px;
    gap: 24px;
  }

  .artwork-wrapper {
    width: 100%;
    max-width: 320px;
    aspect-ratio: 1;
  }

  .artwork-shadow {
    width: 100%;
    height: 100%;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 20px 50px rgba(0,0,0,0.8);
    background: #1a1a1a;
  }

  .main-art {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .placeholder-art {
    width: 100%;
    height: 100%;
    display: grid;
    place-items: center;
    font-size: 5rem;
  }

  .meta-section {
    width: 100%;
    max-width: 400px;
  }

  .title-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 24px;
  }

  .text-meta {
    flex: 1;
    min-width: 0;
  }

  .text-meta h1 {
    font-size: 1.6rem;
    font-weight: 800;
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .text-meta h2 {
    font-size: 1.1rem;
    color: rgba(255, 255, 255, 0.6);
    font-weight: 500;
    margin: 4px 0 0 0;
  }

  .progress-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .progress-bar-bg {
    width: 100%;
    height: 4px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-bar-fill {
    height: 100%;
    background: white;
    transition: width 0.3s linear;
  }

  .time-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 500;
  }

  .controls-section {
    width: 100%;
    max-width: 400px;
  }

  .player-footer {
    padding: 16px;
    display: flex;
    justify-content: center;
    background: linear-gradient(to top, rgba(0,0,0,0.8), transparent);
  }

  .footer-action {
    background: rgba(255, 255, 255, 0.1);
    border: none;
    color: white;
    padding: 8px 24px;
    border-radius: 20px;
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .icon-btn {
    background: none;
    border: none;
    color: white;
    padding: 8px;
    cursor: pointer;
  }

  :global(.controls-card) {
    background: transparent !important;
    border: none !important;
    padding: 0 !important;
  }
</style>
