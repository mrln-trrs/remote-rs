import { writable } from 'svelte/store'

import { getBackendWsUrl, getMedia, sendControlCommand, setVolume } from '../api/backend'

export type MediaSnapshot = {
  title: string
  artist: string
  album: string
  source: string
  artworkUrl: string
  volume: number
  isPlaying: boolean
  progress: number
  duration: number
}

const defaultMedia: MediaSnapshot = {
  title: 'No media',
  artist: 'Unknown',
  album: 'Waiting for laptop',
  source: 'Remote RS',
  artworkUrl: '',
  volume: 50,
  isPlaying: false,
  progress: 0,
  duration: 0,
}

export const media = writable<MediaSnapshot>(defaultMedia)

let pollHandle: ReturnType<typeof setInterval> | undefined
let reconnectHandle: ReturnType<typeof setTimeout> | undefined
let liveSocket: WebSocket | undefined
let reconnectDelay = 500
let cachedSnapshot = defaultMedia
let snapshotCacheLoaded = false

function getCacheKey() {
  return 'remote-rs:media-snapshot:v1'
}

function readSnapshotCache() {
  if (snapshotCacheLoaded || typeof window === 'undefined') {
    snapshotCacheLoaded = true
    return cachedSnapshot
  }

  snapshotCacheLoaded = true

  try {
    const raw = window.localStorage.getItem(getCacheKey())

    if (!raw) {
      return cachedSnapshot
    }

    const parsed = JSON.parse(raw) as Partial<MediaSnapshot>
    cachedSnapshot = {
      ...defaultMedia,
      ...parsed,
      isPlaying: Boolean(parsed.isPlaying),
    }
  } catch {
    cachedSnapshot = defaultMedia
  }

  return cachedSnapshot
}

function writeSnapshotCache(snapshot: MediaSnapshot) {
  cachedSnapshot = snapshot

  if (typeof window === 'undefined') {
    return
  }

  try {
    window.localStorage.setItem(getCacheKey(), JSON.stringify(snapshot))
  } catch {
    // Ignore storage quota and privacy mode failures.
  }
}

media.set(defaultMedia)

function applySnapshot(snapshot: MediaSnapshot) {
  media.set(snapshot)
}

function toNumber(value: unknown, fallback: number) {
  const parsed = typeof value === 'number' ? value : Number(value)
  return Number.isFinite(parsed) ? parsed : fallback
}

function toString(value: unknown, fallback: string) {
  return typeof value === 'string' && value.trim() ? value : fallback
}

function toBoolean(value: unknown, fallback: boolean) {
  if (typeof value === 'boolean') {
    return value
  }

  if (typeof value === 'number') {
    return value !== 0
  }

  if (typeof value === 'string') {
    const normalized = value.trim().toLowerCase()

    if (normalized === 'true' || normalized === '1' || normalized === 'playing') {
      return true
    }

    if (normalized === 'false' || normalized === '0' || normalized === 'paused') {
      return false
    }
  }

  return fallback
}

function inferArtworkMime(value: string) {
  if (value.startsWith('iVBORw0KGgo')) {
    return 'image/png'
  }

  if (value.startsWith('/9j/')) {
    return 'image/jpeg'
  }

  if (value.startsWith('R0lGOD')) {
    return 'image/gif'
  }

  if (value.startsWith('UklGR')) {
    return 'image/webp'
  }

  return 'image/jpeg'
}

function normalizeArtworkSource(value: string) {
  const trimmed = value.trim()

  if (!trimmed) {
    return ''
  }

  if (/^data:image\//i.test(trimmed) || /^https?:\/\//i.test(trimmed) || /^blob:/i.test(trimmed)) {
    return trimmed
  }

  if (trimmed.startsWith('/') || trimmed.startsWith('./') || trimmed.startsWith('../')) {
    return trimmed
  }

  const compact = trimmed.replace(/\s+/g, '')

  if (/^[A-Za-z0-9+/=]+$/.test(compact) && compact.length > 64) {
    return `data:${inferArtworkMime(compact)};base64,${compact}`
  }

  return trimmed
}

function resolveArtwork(payload: Record<string, unknown>) {
  const rawArtwork =
    toString(payload.thumbnail, '') ||
    toString(payload.artworkUrl, '') ||
    toString(payload.coverUrl, '') ||
    toString(payload.thumbnailUrl, '') ||
    toString(payload.image, '') ||
    toString(payload.cover, '')

  return normalizeArtworkSource(rawArtwork)
}

function normalizeMedia(payload: Record<string, unknown>): MediaSnapshot {
  return {
    title: toString(payload.title, defaultMedia.title),
    artist: toString(payload.artist, defaultMedia.artist),
    album: toString(payload.album, defaultMedia.album),
    source: toString(payload.source, defaultMedia.source),
    artworkUrl: resolveArtwork(payload),
    volume: toNumber(payload.volume, defaultMedia.volume),
    isPlaying: toBoolean(payload.isPlaying ?? payload.is_playing ?? payload.playing, defaultMedia.isPlaying),
    progress: toNumber(payload.progress, defaultMedia.progress),
    duration: toNumber(payload.duration, defaultMedia.duration),
  }
}

export async function refreshMedia() {
  try {
    const payload = await getMedia()

    if (!payload) {
      return
    }

    applySnapshot({ ...defaultMedia, ...normalizeMedia(payload) })
  } catch (error) {
    console.error('Failed to fetch media info', error)
  }
}

export async function sendMediaCommand(command: string) {
  try {
    if (command === 'playpause') {
      media.update((current) => ({
        ...current,
        isPlaying: !current.isPlaying,
      }))
    }

    await sendControlCommand(command)
  } catch (error) {
    console.error('Failed to send media command', error)
  }
}

export async function updateVolume(level: number) {
  try {
    // Optimistic UI: Update local state immediately
    media.update(m => ({ ...m, volume: level }));
    await setVolume(level)
  } catch (error) {
    console.error('Failed to update volume', error)
  }
}

function endMediaPolling() {
  if (!pollHandle) {
    return
  }

  clearInterval(pollHandle)
  pollHandle = undefined
}

function beginMediaPolling(interval = 2500) {
  if (pollHandle) {
    return
  }

  void refreshMedia()
  pollHandle = setInterval(refreshMedia, interval)
}

function clearReconnectTimer() {
  if (!reconnectHandle) {
    return
  }

  clearTimeout(reconnectHandle)
  reconnectHandle = undefined
}

function scheduleReconnect() {
  if (typeof window === 'undefined') {
    return
  }

  clearReconnectTimer()
  reconnectHandle = setTimeout(connectLiveSocket, reconnectDelay)
  reconnectDelay = Math.min(reconnectDelay * 1.5, 5000)
}

function extractMediaFromSocketPayload(payload: unknown) {
  if (!payload || typeof payload !== 'object') {
    return undefined
  }

  const record = payload as Record<string, unknown>

  if (record.media && typeof record.media === 'object') {
    return record.media as Record<string, unknown>
  }

  return record
}

function connectLiveSocket() {
  if (typeof window === 'undefined') {
    return
  }

  // Local timer to smooth out progress updates
  setInterval(() => {
    media.update(m => {
      if (m.isPlaying && m.progress < m.duration) {
        return { ...m, progress: m.progress + 1000 };
      }
      return m;
    });
  }, 1000);

  try {
    const socket = new WebSocket(getBackendWsUrl('/ws'))
    liveSocket = socket

    socket.onopen = () => {
      reconnectDelay = 500
      endMediaPolling()
    }

    socket.onmessage = (event) => {
      try {
        const payload = extractMediaFromSocketPayload(JSON.parse(event.data))

        if (!payload) {
          return
        }

        applySnapshot({ ...defaultMedia, ...normalizeMedia(payload) })
      } catch (error) {
        console.error('Failed to parse live media payload', error)
      }
    }

    socket.onerror = () => {
      try {
        socket.close()
      } catch {
        // Ignore close failures.
      }
    }

    socket.onclose = () => {
      if (liveSocket === socket) {
        liveSocket = undefined
      }

      beginMediaPolling()
      scheduleReconnect()
    }
  } catch (error) {
    console.error('Failed to open live media socket', error)
    beginMediaPolling()
    scheduleReconnect()
  }
}

export function startMediaRealtimeSync() {
  void refreshMedia()
  connectLiveSocket()

  return stopMediaRealtimeSync
}

export function stopMediaRealtimeSync() {
  endMediaPolling()
  clearReconnectTimer()

  if (!liveSocket) {
    return
  }

  try {
    liveSocket.close()
  } finally {
    liveSocket = undefined
  }
}

export function startMediaPolling(interval = 2500) {
  if (pollHandle) {
    return stopMediaPolling
  }

  void refreshMedia()
  pollHandle = setInterval(refreshMedia, interval)

  return stopMediaPolling
}

export function stopMediaPolling() {
  if (!pollHandle) {
    return
  }

  clearInterval(pollHandle)
  pollHandle = undefined
}