<script lang="ts">
  import { media } from './stores/media'

  let artworkFailed = false

  $: artworkSource = $media.artworkUrl
  $: if (artworkSource) {
    artworkFailed = false
  }
</script>

<section class="now-playing surface">
  <div class="now-playing__body">
    <div class="artwork">
      {#if artworkSource && !artworkFailed}
        <img src={artworkSource} alt={`Caratula de ${$media.title}`} on:error={() => (artworkFailed = true)} />
      {:else}
        <div class="artwork__fallback">
          <span>🎧</span>
          <small>Sin caratula</small>
        </div>
      {/if}
    </div>

    <div class="meta">
      <h3>{$media.title}</h3>
      <p class="meta__artist">{$media.artist}</p>

      {#if $media.duration > 0}
        <div class="progress">
          <div class="progress__track" aria-hidden="true">
            <div class="progress__fill" style={`width: ${Math.min(($media.progress / $media.duration) * 100, 100)}%`}></div>
          </div>
        </div>
      {/if}
    </div>
  </div>
</section>

<style>
  .now-playing {
    min-height: 0;
    padding: clamp(0.8rem, 2vw, 1rem);
    display: grid;
    gap: 0;
  }

  .now-playing__body {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: flex-start;
    gap: clamp(12px, 2vw, 18px);
    min-height: 0;
  }

  .artwork {
    aspect-ratio: 1;
    border-radius: 0;
    overflow: hidden;
    background: rgba(0, 0, 0, 0.15);
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: none;
    flex: 1 1 18rem;
    width: 100%;
    max-width: min(100%, 24rem);
    margin-inline: auto;
  }

  .artwork img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .artwork__fallback {
    height: 100%;
    display: grid;
    place-items: center;
    gap: 6px;
    color: var(--text-strong);
    text-align: center;
    background: transparent;
  }

  .artwork__fallback span {
    font-size: 2.5rem;
  }

  .artwork__fallback small {
    color: var(--muted);
  }

  .meta {
    display: flex;
    flex: 1 1 16rem;
    min-width: 0;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    gap: 0.4rem;
    padding: 6px 2px;
  }

  h3 {
    font-size: clamp(1.35rem, 4.6vw, 2rem);
    line-height: 1.05;
    color: var(--text-strong);
    letter-spacing: -0.05em;
  }

  .meta__artist {
    color: var(--muted);
  }

  .progress {
    display: grid;
    gap: 0;
    margin-top: 0.5rem;
  }

  .progress__track {
    width: 100%;
    height: 6px;
    border-radius: 0;
    background: rgba(255, 255, 255, 0.12);
    overflow: hidden;
    border: none;
  }

  .progress__fill {
    height: 100%;
    border-radius: 0;
    background: linear-gradient(90deg, var(--accent), var(--accent-2));
    box-shadow: none;
  }

  @media (min-width: 860px) {
    .now-playing__body {
      flex-wrap: nowrap;
    }
  }

  @media (max-width: 859px) {
    .artwork {
      max-height: 42svh;
      max-width: 100%;
      flex-basis: 100%;
    }

    .meta {
      flex-basis: 100%;
      align-items: center;
      text-align: center;
    }
  }
</style>