import { Music2, Volume2, Terminal, Play, Shield, MousePointer2 } from '@lucide/svelte'

export type LauncherItem = {
  id: string
  title: string
  description: string
  routePath: string
  icon: any
}

export const launcherItems: LauncherItem[] = [
  {
    id: 'player',
    title: 'Reproductor',
    description: 'Control de música',
    routePath: '/player',
    icon: Music2,
  },
  {
    id: 'volume',
    title: 'Mezclador',
    description: 'Niveles de audio',
    routePath: '/controls',
    icon: Volume2,
  },
  {
    id: 'youtube',
    title: 'YouTube',
    description: 'Búsqueda remota',
    routePath: '/', // Will implement a page later
    icon: Play,
  },
  {
    id: 'terminal',
    title: 'Terminal',
    description: 'Comandos remotos',
    routePath: '/', 
    icon: Terminal,
  },
  {
    id: 'mouse',
    title: 'Touchpad',
    description: 'Control de cursor',
    routePath: '/',
    icon: MousePointer2,
  },
  {
    id: 'system',
    title: 'Sistema',
    description: 'Energía y estado',
    routePath: '/system',
    icon: Shield,
  },
]