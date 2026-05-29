import { Music2, Volume2 } from '@lucide/svelte'

export type LauncherItem = {
  id: string
  title: string
  description: string
  targetId: string
  routePath: string
  icon: typeof Music2
}

export const launcherItems: LauncherItem[] = [
  {
    id: 'player',
    title: 'Música',
    description: 'Carátula y reproducción',
    targetId: 'now-playing',
    routePath: '/player',
    icon: Music2,
  },
  {
    id: 'volume',
    title: 'Volumen',
    description: 'Audio rápido',
    targetId: 'controls',
    routePath: '/controls',
    icon: Volume2,
  },
]