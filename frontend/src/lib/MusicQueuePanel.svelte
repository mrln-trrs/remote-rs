<script lang="ts">
  import { media } from './stores/media'

  type QueueItem = {
    title: string
    artist: string
    state: string
    kind: 'current' | 'next'
  }

  let { activeQueue = [] } = $props<{ activeQueue?: QueueItem[] }>()

  const currentTrack = $derived({
    title: $media.title,
    artist: $media.artist,
    state: $media.isPlaying ? 'Reproduciendo' : 'Pausado',
    kind: 'current' as const,
  })

  const queue = $derived(activeQueue.length ? activeQueue : [currentTrack])
</script>

<section class="queue-card surface">
  <div class="queue-card__head">
    <p class="eyebrow">Lista</p>
    <h3>Actual y siguientes</h3>
  </div>

  <div class="queue-card__list">
    <article class:queue-card__item--current={currentTrack.kind === 'current'} class="queue-card__item">
      <div class="queue-card__meta">
        <span class="queue-card__label">Actual</span>
        <strong>{currentTrack.title}</strong>
        <small>{currentTrack.artist}</small>
      </div>
      <span class="queue-card__state">{currentTrack.state}</span>
    </article>

    <div class="queue-card__divider" aria-hidden="true"></div>

    <div class="queue-card__next">
      <div class="queue-card__next-head">
        <span class="queue-card__label">Siguientes</span>
        <small>{queue.length > 1 ? `${queue.length - 1} pendientes` : 'Sin cola detectada'}</small>
      </div>

      <div class="queue-card__next-list">
        {#if queue.length > 1}
          {#each queue.slice(1) as item}
            <article class="queue-card__next-item">
              <strong>{item.title}</strong>
              <small>{item.artist}</small>
              <span>{item.state}</span>
            </article>
          {/each}
        {:else}
          <article class="queue-card__next-item queue-card__next-item--empty">
            <strong>Lista vacía</strong>
            <small>El backend todavía no expone una cola.</small>
          </article>
        {/if}
      </div>
    </div>
  </div>
</section>

<style>
  .queue-card {
    padding: clamp(0.85rem, 2vw, 1rem);
    display: grid;
    gap: 0.95rem;
  }

  .queue-card__head {
    display: grid;
    gap: 0.35rem;
  }

  .queue-card__head h3 {
    font-size: clamp(1.05rem, 3vw, 1.35rem);
    line-height: 1.05;
    color: var(--text-strong);
    letter-spacing: -0.04em;
  }

  .queue-card__list {
    display: grid;
    gap: 0.85rem;
  }

  .queue-card__item {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    border: 1px solid var(--border);
    padding: 0.85rem;
    background: rgba(255, 255, 255, 0.02);
  }

  .queue-card__item--current {
    border-color: rgba(69, 215, 255, 0.28);
    background: rgba(69, 215, 255, 0.05);
  }

  .queue-card__meta {
    display: grid;
    gap: 0.2rem;
    min-width: 0;
  }

  .queue-card__label {
    font-size: 0.72rem;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: var(--accent-2);
  }

  .queue-card__meta strong,
  .queue-card__next-item strong {
    color: var(--text-strong);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .queue-card__meta small,
  .queue-card__next-item small,
  .queue-card__state,
  .queue-card__next-head small,
  .queue-card__next-item span {
    color: var(--muted);
  }

  .queue-card__state {
    flex: 0 0 auto;
  }

  .queue-card__divider {
    height: 1px;
    background: rgba(255, 255, 255, 0.08);
  }

  .queue-card__next {
    display: grid;
    gap: 0.65rem;
  }

  .queue-card__next-head {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    align-items: center;
  }

  .queue-card__next-list {
    display: grid;
    gap: 0.6rem;
  }

  .queue-card__next-item {
    display: grid;
    gap: 0.15rem;
    padding: 0.7rem 0.8rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.02);
  }

  .queue-card__next-item--empty {
    border-style: dashed;
  }

  @media (max-width: 599px) {
    .queue-card__item {
      flex-direction: column;
      align-items: flex-start;
    }

    .queue-card__next-head {
      flex-direction: column;
      align-items: flex-start;
    }
  }
</style>