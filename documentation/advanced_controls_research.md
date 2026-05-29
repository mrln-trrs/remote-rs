# Research: Advanced Media Control & Universal Search

## 1. Spotify Playlist Integration
**Challenge:** Windows GSMTC only reports the *current* track. It doesn't know about the queue or playlists.
**Solutions:**
- **Official Spotify Web API:** Requires the user to log in (OAuth2). This allows full control over playlists, queues, and search.
- **Local Scraping (Hard):** Spotify's desktop app doesn't expose its internal state easily via Win32.
**Plan:** We will implement an optional Spotify integration using their Web API. This will allow:
- Viewing current playlist/queue.
- Searching for songs/artists.
- Switching playback devices.

## 2. Web Media (YouTube / YT Music)
**Challenge:** Browsers report current track info to Windows, but not the "Up Next" list.
**Solutions:**
- **Browser Extension:** A small Chrome/Edge extension could "pipe" the YouTube queue data to our Rust backend via WebSockets.
- **URL Handling:** We can "Search" by sending a URL to the laptop to open in the default browser.
    - Example: `POST /api/open?url=https://youtube.com/results?search_query=...`

## 3. Universal Search & Launcher
**Concept:** A search bar in the mobile app that can:
- Search Spotify (via API).
- Search YouTube (opens browser on laptop).
- Search local files (optional).
- Launch specific Apps (e.g., "Open Calculator").

## 4. Technical Strategy
- **New Module:** `src/modules/launcher/` for opening URLs and Apps.
- **New Module:** `src/modules/spotify/` for API integration.
- **Optimization:** These features will remain "on-demand" to maintain the zero-consumption goal.
