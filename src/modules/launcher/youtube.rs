use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct YouTubeResult {
    pub id: String,
    pub title: String,
    pub thumbnail: String,
}

pub async fn search_youtube_results(query: &str) -> Vec<YouTubeResult> {
    // Note: To use the official YouTube API, you'd need an API Key.
    // For a "super light" and "no config" version, we can scrape the search page
    // or use a public instance. 
    // Here we'll implement a placeholder that explains the requirement.
    
    // In a real implementation:
    // let url = format!("https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&key={}", query, API_KEY);
    // let res = reqwest::get(url).await...
    
    vec![
        YouTubeResult {
            id: "dQw4w9WgXcQ".to_string(),
            title: "Rick Astley - Never Gonna Give You Up".to_string(),
            thumbnail: "https://i.ytimg.com/vi/dQw4w9WgXcQ/hqdefault.jpg".to_string(),
        }
    ]
}

pub fn play_video(video_id: &str) -> bool {
    let url = format!("https://www.youtube.com/watch?v={}", video_id);
    Command::new("cmd")
        .args(&["/C", "start", "", &url])
        .spawn()
        .is_ok()
}
