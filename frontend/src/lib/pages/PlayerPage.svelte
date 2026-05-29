<script lang="ts">
  import NowPlayingCard from '../NowPlayingCard.svelte'
  import MusicQueuePanel from '../MusicQueuePanel.svelte'
  import MediaControl from '../MediaControl.svelte'
</script>

<section class="page-frame">
  <button type="button" class="page-frame__back" onclick={() => history.back()} aria-label="Volver">
    <span aria-hidden="true">←</span>
    <span>Volver</span>
  </button>

  <div class="page-frame__stack">
    <NowPlayingCard />
    <MediaControl />
    <MusicQueuePanel />
  </div>
</section>

<style>
  .page-frame {
    width: 100%;
    min-height: 0;
    display: flex;
    flex-direction: column;
    gap: clamp(12px, 2.2vw, 18px);
  }

  .page-frame__back {
    appearance: none;
    border: 1px solid var(--border);
    border-radius: 0;
    background: transparent;
    color: var(--text-strong);
    padding: 0.7rem 1rem;
    display: inline-flex;
    align-items: center;
    gap: 10px;
    width: fit-content;
    cursor: pointer;
    box-shadow: none;
  }

  .page-frame__stack {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: flex-start;
    gap: clamp(12px, 2.2vw, 18px);
    min-height: 0;
    flex: 0 0 auto;
  }

  .page-frame__stack :global(.now-playing),
  .page-frame__stack :global(.controls-card),
  .page-frame__stack :global(.queue-card) {
    flex: 1 1 22rem;
    width: min(100%, 30rem);
    max-width: 100%;
  }

  @media (max-width: 599px) {
    .page-frame__stack :global(.now-playing),
    .page-frame__stack :global(.controls-card),
    .page-frame__stack :global(.queue-card) {
      width: 100%;
      flex-basis: 100%;
    }
  }
</style>