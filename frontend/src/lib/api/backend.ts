const defaultBackendUrl = 'http://localhost:3000'

const mediaCache = new Map<string, MediaPayload | undefined>()
const pendingRequests = new Map<string, Promise<unknown>>()

function getBackendUrl() {
  if (typeof window !== 'undefined') {
    return `http://${window.location.hostname}:3000`
  }
  const envUrl = import.meta.env.VITE_REMOTE_RS_API_URL
  return typeof envUrl === 'string' && envUrl.trim() ? envUrl.replace(/\/$/, '') : defaultBackendUrl
}

export function getBackendWsUrl(path: string) {
  const url = new URL(getBackendUrl())

  url.protocol = url.protocol === 'https:' ? 'wss:' : 'ws:'
  url.pathname = path.startsWith('/') ? path : `/${path}`
  url.search = ''
  url.hash = ''

  return url.toString().replace(/\/$/, '')
}

async function requestJson<T>(path: string, init?: RequestInit) {
  const requestPromise = (async () => {
    const response = await fetch(`${getBackendUrl()}${path}`, init)
    if (!response.ok) {
      throw new Error(`Request failed for ${path}: ${response.status}`)
    }
    if (response.status === 204) {
      return undefined as T
    }
    const body = await response.text()
    if (!body.trim()) {
      return undefined as T
    }
    return JSON.parse(body) as T
  })()

  return requestPromise
}

export type MediaPayload = {
  title?: string
  artist?: string
  volume?: number
  is_playing?: boolean
  playing?: boolean
  source?: string
  artworkUrl?: string
  coverUrl?: string
  thumbnailUrl?: string
  thumbnail?: string
  image?: string
  cover?: string
}

export async function getMedia() {
  return requestJson<MediaPayload>('/api/media')
}

export async function sendControlCommand(command: string) {
  await requestJson<void>(`/api/control/${encodeURIComponent(command)}`, {
    method: 'POST',
  })
}

export async function setVolume(level: number) {
  await requestJson<void>(`/api/volume/${encodeURIComponent(String(level))}`, {
    method: 'POST',
  })
}

export async function sendSystemAction(action: 'lock' | 'shutdown' | 'reboot' | 'sleep') {
  await requestJson<void>(`/api/system/${encodeURIComponent(action)}`, {
    method: 'POST',
  })
}
