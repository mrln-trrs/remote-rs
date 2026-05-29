use enigo::{Enigo, Key, Keyboard, Settings};

pub fn send_media_key(key: &str) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    
    let enigo_key = match key {
        "playpause" => Key::MediaPlayPause,
        "next" => Key::MediaNextTrack,
        "prev" => Key::MediaPrevTrack,
        "volup" => Key::VolumeUp,
        "voldown" => Key::VolumeDown,
        _ => return,
    };

    let _ = enigo.key(enigo_key, enigo::Direction::Click);
}
