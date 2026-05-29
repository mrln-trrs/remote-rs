# Flow: Remote YouTube Search & Playback

To allow the user to search AND pick a video from the mobile device to play on the laptop, we will implement the following flow:

## 1. Search Phase (Mobile-centric)
Instead of searching on the laptop and trying to "see" the results on the mobile, we do the inverse:
1.  **Mobile App:** User types a query in the search bar.
2.  **Request:** The mobile app calls the YouTube Search API (either directly or proxied through our Rust backend).
3.  **Display:** The mobile app displays a list of videos with thumbnails, titles, and channel names.

## 2. Selection Phase
1.  **Mobile App:** User taps on a video from the list.
2.  **Request:** Mobile app sends the specific Video ID or URL to the Rust backend:
    - `POST /api/launch/url` with `{ "url": "https://www.youtube.com/watch?v=VIDEO_ID" }`

## 3. Playback Phase (Laptop)
1.  **Backend (Rust):** Receives the URL.
2.  **Native Command:** Uses `cmd /C start https://...` to open the video in the laptop's default browser.
3.  **Browser:** YouTube starts playing the video.

## 4. Control Phase
Once the video is playing, the existing **Media Module** takes over:
- Play/Pause, Next/Prev, and Volume will work via the GSMTC integration already implemented.

---

# Backend Enhancement: Search Proxy
To avoid exposing API keys in the frontend or handling complex OAuth, the Rust backend will act as a lightweight proxy for YouTube searches.

## API Endpoint
`GET /api/search/youtube?q=query`
- Returns a clean JSON list of results (Video ID, Title, Thumbnail URL).
