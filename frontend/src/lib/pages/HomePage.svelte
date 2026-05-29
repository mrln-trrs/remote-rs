<script lang="ts">
  import { Pause, Play, Volume1, Volume2, VolumeX } from '@lucide/svelte'

  import AppLauncher from '../AppLauncher.svelte'
  import { launcherItems } from '../data/launcher'
  import { media } from '../stores/media'
  import { goTo } from '../stores/router'

  function volumeIcon(volume: number) {
    if (volume === 0) {
      return VolumeX
    }

    if (volume < 35) {
      return Volume1
    }

    return Volume2
  }

  $: homeLaunchers = launcherItems.map((item) => {
    if (item.id === 'player') {
      return {
        ...item,
        icon: $media.isPlaying ? Pause : Play,
        description: $media.isPlaying ? `Sonando: ${$media.title}` : 'Toca para abrir',
      }
    }

    return {
      ...item,
      icon: volumeIcon($media.volume),
      description: $media.isPlaying ? $media.artist : 'Audio rápido',
    }
  })
</script>

<section class="page-frame">
  <div class="page-frame__intro">
    <p class="eyebrow">Remote RS</p>
    <h2>Inicio</h2>
  </div>

  <div class="page-frame__grid">
    <AppLauncher items={homeLaunchers} activeId={$media.isPlaying ? 'player' : 'volume'} onSelect={(item) => goTo(item.routePath)} />
  </div>
</section>

<style>
  .page-frame {
    height: 100%;
    min-height: 0;
    display: grid;
    grid-template-rows: auto 1fr;
    gap: 14px;
  }

  .page-frame__intro {
    display: flex;
    flex-direction: column;
    gap: 4px;
    align-content: start;
  }

  .page-frame__grid {
    display: grid;
    gap: 0;
    min-height: 0;
    align-content: start;
    overflow: hidden;
  }
</style>