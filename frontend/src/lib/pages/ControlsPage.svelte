<script lang="ts">
  import { SlidersHorizontal, Settings2, Volume2 } from '@lucide/svelte'

  import PageHeader from '../PageHeader.svelte'
  import { media, updateVolume } from '../stores/media'

  type VolumePanel = 'mix' | 'eq'

  let activePanel: VolumePanel = 'mix'

  const panelActions = [
    {
      id: 'mix' as const,
      title: 'Mezcla de volúmenes',
      subtitle: 'Ajuste maestro y futuros canales de audio',
      icon: SlidersHorizontal,
    },
    {
      id: 'eq' as const,
      title: 'Ecualización',
      subtitle: 'Base para programar perfiles y bandas',
      icon: Settings2,
    },
  ]

  function handleMasterVolumeInput(event: Event) {
    const input = event.currentTarget as HTMLInputElement | null

    if (!input) {
      return
    }

    void updateVolume(Number(input.value))
  }
</script>

<section class="page-frame">
  <PageHeader
    eyebrow="Controles"
    title="Volumen"
  />

  <div class="page-frame__body">
    {#if activePanel === 'mix'}
      <section class="surface volume-panel">
        <div class="volume-panel__top">
          <span class="volume-panel__label">Mezcla</span>
          <strong>{$media.volume}%</strong>
        </div>

        <div class="volume-panel__mixer">
          <div class="volume-panel__mixer-head">
            <span>Master</span>
            <Volume2 size={18} strokeWidth={2.1} aria-hidden="true" />
          </div>

          <input
            type="range"
            min="0"
            max="100"
            value={$media.volume}
            aria-label="Volumen maestro"
            oninput={handleMasterVolumeInput}
          />
        </div>
      </section>
    {:else}
      <section class="surface volume-panel volume-panel--eq">
        <div class="volume-panel__top">
          <span class="volume-panel__label">EQ</span>
          <strong>Listo</strong>
        </div>

        <div class="eq-visual" aria-hidden="true">
          <span></span>
          <span></span>
          <span></span>
          <span></span>
          <span></span>
          <span></span>
          <span></span>
        </div>
      </section>
    {/if}

    <div class="page-frame__actions">
      {#each panelActions as panel}
        {@const Icon = panel.icon}
        <button
          type="button"
          class:active={activePanel === panel.id}
          class="panel-action"
          onclick={() => (activePanel = panel.id)}
        >
          <span class="panel-action__icon" aria-hidden="true">
            <Icon size={22} strokeWidth={2.1} />
          </span>
          <span class="panel-action__copy">
            <strong>{panel.title}</strong>
            <small>{panel.subtitle}</small>
          </span>
        </button>
      {/each}
    </div>
  </div>
</section>

<style>
  .page-frame {
    width: 100%;
    min-height: 0;
    display: grid;
    gap: clamp(12px, 2vw, 18px);
  }

  .page-frame__body {
    display: grid;
    gap: clamp(12px, 2vw, 18px);
    min-height: 0;
  }

  .volume-panel {
    padding: clamp(0.85rem, 2vw, 1rem);
    display: grid;
    gap: 1rem;
  }

  .volume-panel__top {
    display: grid;
    gap: 0.35rem;
  }

  .volume-panel__label {
    font-size: 0.78rem;
    text-transform: uppercase;
    letter-spacing: 0.22em;
    color: var(--accent-2);
  }

  .volume-panel__mixer {
    display: grid;
    gap: 0.75rem;
  }

  .volume-panel__mixer-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    color: var(--text-strong);
  }

  .volume-panel input[type='range'] {
    width: 100%;
    appearance: none;
    background: transparent;
    height: 28px;
  }

  .volume-panel input[type='range']::-webkit-slider-runnable-track {
    height: 6px;
    border-radius: 999px;
    background: linear-gradient(90deg, var(--accent), var(--accent-2));
  }

  .volume-panel input[type='range']::-webkit-slider-thumb {
    appearance: none;
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid rgba(6, 6, 8, 0.95);
    background: var(--accent-2);
    margin-top: -6px;
    box-shadow: 0 0 0 6px rgba(69, 215, 255, 0.08);
  }

  .volume-panel input[type='range']::-moz-range-track {
    height: 6px;
    border-radius: 999px;
    background: linear-gradient(90deg, var(--accent), var(--accent-2));
  }

  .volume-panel input[type='range']::-moz-range-thumb {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(6, 6, 8, 0.95);
    border-radius: 50%;
    background: var(--accent-2);
    box-shadow: 0 0 0 6px rgba(69, 215, 255, 0.08);
  }

  .volume-panel--eq {
    min-height: 9rem;
  }

  .eq-visual {
    display: grid;
    grid-template-columns: repeat(7, minmax(0, 1fr));
    gap: 0.5rem;
    align-items: end;
    min-height: 6.5rem;
  }

  .eq-visual span {
    display: block;
    min-height: 1.25rem;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.14), rgba(255, 255, 255, 0.04));
  }

  .eq-visual span:nth-child(1) { height: 22%; }
  .eq-visual span:nth-child(2) { height: 48%; }
  .eq-visual span:nth-child(3) { height: 76%; }
  .eq-visual span:nth-child(4) { height: 40%; }
  .eq-visual span:nth-child(5) { height: 68%; }
  .eq-visual span:nth-child(6) { height: 30%; }
  .eq-visual span:nth-child(7) { height: 58%; }

  .page-frame__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
  }

  .panel-action {
    appearance: none;
    border: 1px solid var(--border);
    background: rgba(255, 255, 255, 0.03);
    color: var(--text-strong);
    min-width: min(100%, 16rem);
    flex: 1 1 16rem;
    padding: 0.95rem;
    display: flex;
    align-items: center;
    gap: 0.85rem;
    cursor: pointer;
    transition:
      transform 160ms ease,
      border-color 160ms ease,
      background 160ms ease;
  }

  .panel-action:hover,
  .panel-action:focus-visible,
  .panel-action.active {
    transform: translateY(-1px);
    border-color: rgba(69, 215, 255, 0.38);
    background: rgba(255, 255, 255, 0.05);
  }

  .panel-action:focus-visible {
    outline: 2px solid rgba(69, 215, 255, 0.65);
    outline-offset: 2px;
  }

  .panel-action__icon {
    width: 2.8rem;
    height: 2.8rem;
    display: grid;
    place-items: center;
    border: 1px solid rgba(255, 255, 255, 0.12);
    background: rgba(255, 255, 255, 0.04);
    flex: 0 0 auto;
  }

  .panel-action__copy {
    display: grid;
    gap: 0.15rem;
    min-width: 0;
  }

  .panel-action__copy strong {
    font-size: 0.95rem;
  }

  .panel-action__copy small {
    color: var(--muted);
  }

  @media (max-width: 599px) {
    .page-frame__actions {
      flex-direction: column;
    }

    .panel-action {
      min-width: 0;
      width: 100%;
    }
  }
</style>