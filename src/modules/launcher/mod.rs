pub mod youtube;

use std::process::Command;

pub fn open_url(url: &str) -> bool {
    // Windows 'start' command equivalent in Rust
    Command::new("cmd")
        .args(&["/C", "start", "", url])
        .spawn()
        .is_ok()
}

pub fn search_youtube(query: &str) -> bool {
    let url = format!("https://www.youtube.com/results?search_query={}", query.replace(" ", "+"));
    open_url(&url)
}
