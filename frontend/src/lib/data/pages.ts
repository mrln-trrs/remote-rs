export type PageTab = {
  title: string
  path: string
  subtitle: string
  glyph: string
}

export const pageTabs: PageTab[] = [
  { title: 'Inicio', path: '/', subtitle: 'Launcher', glyph: '⌂' },
  { title: 'Música', path: '/player', subtitle: 'Carátula', glyph: '▶' },
  { title: 'Controles', path: '/controls', subtitle: 'Acciones', glyph: '⏯' },
  { title: 'Sistema', path: '/system', subtitle: 'Estado', glyph: '◫' },
]