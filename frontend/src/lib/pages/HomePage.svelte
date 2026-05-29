<script lang="ts">
  import AppLauncher from '../AppLauncher.svelte'
  import { launcherItems } from '../data/launcher'
  import { goTo } from '../stores/router'
  import { media } from '../stores/media'

  $: displayItems = launcherItems.map(item => {
    if (item.id === 'player' && $media.isPlaying) {
      return { ...item, description: `Ahora: ${$media.title}` }
    }
    return item
  })
</script>

<div class="home-page">
  <header class="home-header">
    <p class="eyebrow">Remote RS</p>
    <h1>Inicio</h1>
  </header>

  <section class="section">
    <h2 class="section-title">Acceso Rápido</h2>
    <AppLauncher items={displayItems} onSelect={(item) => goTo(item.routePath)} />
  </section>

  <section class="section">
    <h2 class="section-title">Estado del Sistema</h2>
    <div class="status-card surface">
      <div class="status-item">
        <span class="label">Batería</span>
        <span class="value">95%</span>
      </div>
      <div class="status-item">
        <span class="label">CPU</span>
        <span class="value">12%</span>
      </div>
    </div>
  </section>
</div>

<style>
  .home-page {
    display: flex;
    flex-direction: column;
    gap: 32px;
  }

  .home-header h1 {
    font-size: 2.2rem;
    font-weight: 800;
    margin-top: 4px;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .section-title {
    font-size: 1.2rem;
    font-weight: 700;
  }

  .status-card {
    padding: 20px;
    border-radius: 12px;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
  }

  .status-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .label {
    font-size: 0.8rem;
    color: rgba(255, 255, 255, 0.4);
  }

  .value {
    font-size: 1.1rem;
    font-weight: 600;
    color: white;
  }
</style>
