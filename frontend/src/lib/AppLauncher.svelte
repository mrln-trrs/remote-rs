<script lang="ts">
  import { scale } from 'svelte/transition'

  import type { LauncherItem } from './data/launcher'

  let { items, activeId = '', onSelect } = $props<{
    items: LauncherItem[]
    activeId?: string
    onSelect: (item: LauncherItem) => void
  }>()
</script>

<div class="launcher-grid">
  {#each items as item}
    {@const Icon = item.icon}
    <button
      type="button"
      class:active={item.id === activeId}
      class="launcher-tile"
      onclick={() => onSelect(item)}
    >
      {#key Icon}
        <span class="launcher-tile__icon" aria-hidden="true" transition:scale={{ start: 0.82, duration: 160 }}>
          <Icon size={30} strokeWidth={2.2} />
        </span>
      {/key}

      <span class="launcher-tile__copy">
        <strong>{item.title}</strong>
        <span>{item.description}</span>
      </span>
    </button>
  {/each}
</div>

<style>
  .launcher-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  .launcher-tile {
    appearance: none;
    border: 1px solid var(--border);
    border-radius: 0;
    background: transparent;
    box-shadow: none;
    padding: 14px;
    aspect-ratio: 1;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: flex-start;
    text-align: left;
    cursor: pointer;
    color: var(--text-strong);
    position: relative;
    overflow: hidden;
    transition:
      transform 160ms ease,
      border-color 160ms ease,
      box-shadow 160ms ease,
      background 160ms ease;
  }

  .launcher-tile::before {
    content: '';
    position: absolute;
    inset: 0;
    pointer-events: none;
    background:
      radial-gradient(circle at top right, rgba(255, 255, 255, 0.08), transparent 38%),
      linear-gradient(180deg, rgba(255, 255, 255, 0.02), transparent 58%);
  }

  .launcher-tile:hover,
  .launcher-tile:focus-visible,
  .launcher-tile.active {
    transform: translateY(-1px);
    border-color: rgba(109, 240, 191, 0.38);
    box-shadow: none;
  }

  .launcher-tile.active {
    background: linear-gradient(180deg, rgba(109, 240, 191, 0.08), rgba(255, 255, 255, 0.02));
  }

  .launcher-tile:focus-visible {
    outline: 2px solid rgba(69, 215, 255, 0.7);
    outline-offset: 2px;
  }

  .launcher-tile__icon {
    width: 52px;
    height: 52px;
    border-radius: 9999px;
    display: grid;
    place-items: center;
    background:
      radial-gradient(circle at 30% 30%, rgba(255, 255, 255, 0.18), rgba(255, 255, 255, 0.04) 58%, rgba(255, 255, 255, 0.02));
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: white;
    box-shadow: none;
    transition:
      transform 160ms ease,
      background 160ms ease,
      border-color 160ms ease;
  }

  .launcher-tile.active .launcher-tile__icon {
    transform: scale(1.04);
    border-color: rgba(109, 240, 191, 0.28);
    background:
      radial-gradient(circle at 30% 30%, rgba(109, 240, 191, 0.28), rgba(255, 255, 255, 0.05) 60%, rgba(255, 255, 255, 0.03));
  }

  .launcher-tile__copy {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .launcher-tile__copy strong {
    font-size: 0.98rem;
    line-height: 1.1;
  }

  .launcher-tile__copy span {
    font-size: 0.84rem;
    color: var(--muted);
  }

  @media (min-width: 860px) {
    .launcher-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .launcher-tile {
      padding: 16px;
    }

    .launcher-tile__icon {
      width: 56px;
      height: 56px;
    }
  }

  @media (max-width: 599px) {
    .launcher-tile {
      padding: 12px;
    }

    .launcher-tile__icon {
      width: 48px;
      height: 48px;
    }

    .launcher-tile__copy strong {
      font-size: 0.92rem;
    }

    .launcher-tile__copy span {
      font-size: 0.78rem;
    }
  }
</style>