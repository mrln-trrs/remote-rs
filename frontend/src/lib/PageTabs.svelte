<script lang="ts">
  import page from 'page'
  import { Home, Music, Sliders, Monitor } from '@lucide/svelte'
  import type { PageTab } from './data/pages'

  let { items, activePath } = $props<{
    items: PageTab[]
    activePath: string
  }>()

  const icons: Record<string, any> = {
    '/': Home,
    '/player': Music,
    '/controls': Sliders,
    '/system': Monitor
  }

  function goTo(path: string) {
    page.show(path)
  }
</script>

<nav class="bottom-nav">
  {#each items as item}
    {@const Icon = icons[item.path] || Music}
    <button
      type="button"
      class:active={item.path === activePath}
      onclick={() => goTo(item.path)}
    >
      <Icon size={24} strokeWidth={item.path === activePath ? 2.5 : 2} />
      <span>{item.title}</span>
    </button>
  {/each}
</nav>

<style>
  .bottom-nav {
    display: flex;
    justify-content: space-around;
    align-items: center;
    height: 64px;
    background: rgba(18, 18, 23, 0.95);
    backdrop-filter: blur(10px);
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    padding-bottom: env(safe-area-inset-bottom);
  }

  button {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    color: rgba(255, 255, 255, 0.5);
    font-size: 0.7rem;
    padding: 8px 12px;
    transition: color 0.2s;
    cursor: pointer;
  }

  button.active {
    color: white;
  }

  button :global(svg) {
    transition: transform 0.2s;
  }

  button:active :global(svg) {
    transform: scale(0.9);
  }
</style>
