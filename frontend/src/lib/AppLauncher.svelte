<script lang="ts">
  import type { LauncherItem } from './data/launcher'

  let { items, onSelect } = $props<{
    items: LauncherItem[]
    onSelect: (item: LauncherItem) => void
  }>()
</script>

<div class="launcher-grid">
  {#each items as item}
    {@const Icon = item.icon}
    <button class="launcher-card surface" onclick={() => onSelect(item)}>
      <div class="icon-box">
        <Icon size={24} strokeWidth={2} />
      </div>
      <div class="text-box">
        <span class="title">{item.title}</span>
        <span class="desc">{item.description}</span>
      </div>
    </button>
  {/each}
</div>

<style>
  .launcher-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    width: 100%;
  }

  .launcher-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 16px;
    border-radius: 12px;
    background: #121217;
    border: 1px solid rgba(255, 255, 255, 0.05);
    text-align: left;
    cursor: pointer;
    transition: background 0.2s, transform 0.1s;
    gap: 12px;
  }

  .launcher-card:active {
    transform: scale(0.96);
    background: #1b1b22;
  }

  .icon-box {
    width: 40px;
    height: 40px;
    display: grid;
    place-items: center;
    background: rgba(69, 215, 255, 0.1);
    color: #45d7ff;
    border-radius: 50%;
  }

  .text-box {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .title {
    color: white;
    font-size: 0.95rem;
    font-weight: 600;
  }

  .desc {
    color: rgba(255, 255, 255, 0.4);
    font-size: 0.75rem;
    line-height: 1.2;
  }
</style>
