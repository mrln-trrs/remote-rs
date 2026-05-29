use serde::Serialize;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use windows::Storage::Streams::DataReader;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Default, Clone)]
pub struct MediaMetadata {
    pub title: String,
    pub artist: String,
    pub volume: u32,
    pub is_playing: bool,
    pub thumbnail: Option<String>,
    pub progress: u32,
    pub duration: u32,
}

fn time_span_to_millis(value: windows::Foundation::TimeSpan) -> u32 {
    let millis = value.Duration / 10_000;

    if millis <= 0 {
        0
    } else if millis > u32::MAX as i64 {
        u32::MAX
    } else {
        millis as u32
    }
}

pub async fn get_current_media() -> MediaMetadata {
    let mut meta = MediaMetadata::default();

    if let Ok(op) = GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
        if let Ok(manager) = op.get() {
            if let Ok(session) = manager.GetCurrentSession() {
                // Get Media Properties
                if let Ok(props_op) = session.TryGetMediaPropertiesAsync() {
                    if let Ok(props) = props_op.get() {
                        meta.title = props.Title().unwrap_or_default().to_string();
                        meta.artist = props.Artist().unwrap_or_default().to_string();

                        // Try to get Thumbnail
                        if let Ok(thumb_ref) = props.Thumbnail() {
                            if let Ok(stream_op) = thumb_ref.OpenReadAsync() {
                                if let Ok(stream) = stream_op.get() {
                                    if let Ok(size) = stream.Size() {
                                        if size > 0 {
                                            if let Ok(reader) = DataReader::CreateDataReader(&stream) {
                                                if let Ok(load_op) = reader.LoadAsync(size as u32) {
                                                    if load_op.get().is_ok() {
                                                        let mut buffer = vec![0u8; size as usize];
                                                        if reader.ReadBytes(&mut buffer).is_ok() {
                                                            meta.thumbnail = Some(general_purpose::STANDARD.encode(buffer));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Get Playback Info
                if let Ok(playback) = session.GetPlaybackInfo() {
                    if let Ok(status) = playback.PlaybackStatus() {
                        meta.is_playing = status.0 == 4;
                    }
                }

                if let Ok(timeline) = session.GetTimelineProperties() {
                    let start_time = timeline.StartTime().ok();
                    let end_time = timeline.EndTime().ok();
                    let position = timeline.Position().ok();

                    if let (Some(start_time), Some(end_time), Some(position)) = (start_time, end_time, position) {
                        let start_ms = time_span_to_millis(start_time) as i64;
                        let end_ms = time_span_to_millis(end_time) as i64;
                        let position_ms = time_span_to_millis(position) as i64;

                        meta.duration = (end_ms - start_ms).max(0) as u32;
                        meta.progress = (position_ms - start_ms).max(0) as u32;

                        if meta.duration > 0 {
                            meta.progress = meta.progress.min(meta.duration);
                        }
                    }
                }
            }
        }
    }

    meta
}
