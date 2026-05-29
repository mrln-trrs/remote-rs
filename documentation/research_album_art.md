# Research: Album Art Extraction

## Objective
Display the cover art of the currently playing media (Spotify, YouTube, etc.) on the mobile frontend.

## Technical Context
Windows GSMTC provides album art through `MediaProperties.Thumbnail`. This is a `SoftwareBitmap` or an `IRandomAccessStreamWithContentType`.

## Implementation Strategy (C++ Shim)
Since WinRT streams are complex to handle in pure Rust FFI, we will:
1.  Use the C++ Shim to access `GlobalSystemMediaTransportControlsSession`.
2.  Retrieve the `Thumbnail` stream.
3.  Read the stream bytes into a buffer.
4.  Encode the bytes as **Base64** in the C++ layer (or return raw bytes and encode in Rust).
5.  Pass the string back to Rust.

## API Change
`MediaMetadata` will include a `thumbnail` field (Base64 string).

```json
{
  "title": "Song Name",
  "artist": "Artist Name",
  "is_playing": true,
  "volume": 50,
  "thumbnail": "data:image/jpeg;base64,/9j/4AAQ..."
}
```

## Potential Challenges
- **Latency:** Reading and encoding large thumbnails every polling interval could be heavy. We will implement basic caching (only update if the title/artist changes).
- **Format:** Thumbnails are usually JPEGs or PNGs. We will prepend the correct Data URI prefix.
