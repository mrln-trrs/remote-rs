import page from 'page'
import { writable } from 'svelte/store'

import { pageTabs } from '../data/pages'

export const currentPath = writable('/')

let routerReady = false

export function initRouter() {
  if (routerReady || typeof window === 'undefined') {
    return
  }

  const routes = new Set(pageTabs.map((tab) => tab.path))

  page.base('')

  routes.forEach((path) => {
    page(path, () => {
      currentPath.set(path)
    })
  })

  page('*', (context) => {
    const nextPath = routes.has(context.path) ? context.path : '/'
    currentPath.set(nextPath)

    if (!routes.has(context.path)) {
      page.show('/')
    }
  })

  routerReady = true
  page.start({ click: true, popstate: true, dispatch: true })

  if (!routes.has(window.location.pathname)) {
    page.show('/')
  }
}

export function goTo(path: string) {
  if (typeof window === 'undefined') {
    return
  }

  page.show(path)
}