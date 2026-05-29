<script lang="ts">
  import { goTo } from './stores/router'

  let { eyebrow, title, text, backPath = '/' } = $props<{
    eyebrow: string
    title: string
    text?: string
    backPath?: string
  }>()
</script>

<header class="page-header">
  <button type="button" class="page-header__back" onclick={() => goTo(backPath)}>
    <span aria-hidden="true">←</span>
    <span>Volver</span>
  </button>

  <div class="page-header__copy">
    <p class="eyebrow">{eyebrow}</p>
    <h2>{title}</h2>
    {#if text}
      <p class="page-header__text">{text}</p>
    {/if}
  </div>
</header>

<style>
  .page-header {
    display: grid;
    gap: 12px;
  }

  .page-header__back {
    appearance: none;
    border: 1px solid var(--border);
    border-radius: 0;
    background: transparent;
    color: var(--text-strong);
    padding: 10px 14px;
    display: inline-flex;
    align-items: center;
    gap: 10px;
    width: fit-content;
    cursor: pointer;
    box-shadow: none;
    transition:
      transform 160ms ease,
      border-color 160ms ease;
  }

  .page-header__back:hover,
  .page-header__back:focus-visible {
    transform: translateY(-1px);
    border-color: rgba(255, 255, 255, 0.22);
    outline: none;
  }

  .page-header__copy {
    display: grid;
    gap: 8px;
  }

  .page-header__text {
    color: var(--muted);
    max-width: 60ch;
  }
</style>