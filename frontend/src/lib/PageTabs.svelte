<script lang="ts">
  import page from 'page'

  import type { PageTab } from './data/pages'

  let { items, activePath } = $props<{
    items: PageTab[]
    activePath: string
  }>()

  function goTo(path: string) {
    page.show(path)
  }
</script>

<nav class="page-tabs" aria-label="Navegación principal">
  {#each items as item}
    <button
      type="button"
      class:active={item.path === activePath}
      class="page-tabs__item"
      onclick={() => goTo(item.path)}
    >
      <span class="page-tabs__glyph">{item.glyph}</span>

      <span class="page-tabs__copy">
        <strong>{item.title}</strong>
        <small>{item.subtitle}</small>
      </span>
    </button>
  {/each}
</nav>

<style>
  .page-tabs {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
  }

  .page-tabs__item {
    appearance: none;
    border: 1px solid var(--border);
    border-radius: 0;
    background: transparent;
    box-shadow: none;
    min-height: 88px;
    padding: 14px;
    display: grid;
    grid-template-columns: 48px minmax(0, 1fr);
    gap: 12px;
    align-items: center;
    text-align: left;
    cursor: pointer;
    transition:
      transform 160ms ease,
      border-color 160ms ease,
      box-shadow 160ms ease;
  }

  .page-tabs__item:hover,
  .page-tabs__item:focus-visible,
  .page-tabs__item.active {
    transform: translateY(-2px);
    border-color: rgba(255, 255, 255, 0.22);
    box-shadow: none;
  }

  .page-tabs__item:focus-visible {
    outline: 2px solid rgba(69, 215, 255, 0.7);
    outline-offset: 2px;
  }

  .page-tabs__glyph {
    width: 48px;
    height: 48px;
    display: grid;
    place-items: center;
    border-radius: 0;
    color: var(--text-strong);
    background: transparent;
    border: 1px solid rgba(255, 255, 255, 0.12);
    box-shadow: none;
    font-size: 1.15rem;
  }

  .page-tabs__copy {
    display: grid;
    gap: 2px;
    min-width: 0;
  }

  .page-tabs__copy strong {
    color: var(--text-strong);
  }

  .page-tabs__copy small {
    color: var(--muted);
  }

  @media (min-width: 860px) {
    .page-tabs {
      grid-template-columns: repeat(4, minmax(0, 1fr));
    }
  }

  @media (max-width: 599px) {
    .page-tabs {
      grid-template-columns: 1fr;
    }
  }
</style>