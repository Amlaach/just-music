use aether_core::{AetherError, AudioFormat, Result, Track, TrackId};
use std::fs::File;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTagKey};
use symphonia::core::probe::Hint;

pub struct MetadataExtractor;

impl MetadataExtractor {
    pub fn extract<P: AsRef<Path>>(path: P) -> Result<Track> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let mut probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| AetherError::Decoder(format!("Probe failed: {}", e)))?;

        let track_info = probed
            .format
            .default_track()
            .ok_or_else(|| AetherError::Decoder("No audio track".into()))?;

        let duration_ms = track_info
            .codec_params
            .n_frames
            .map(|f| {
                let sr = track_info.codec_params.sample_rate.unwrap_or(44100) as u64;
                (f * 1000) / sr
            })
            .unwrap_or(0);

        let sample_rate = track_info.codec_params.sample_rate.unwrap_or(44100);
        let channels = track_info.codec_params.channels.map(|c| c.count() as u16).unwrap_or(2);
        let bitrate = track_info.codec_params.bits_per_sample;

        let mut title = None;
        let mut artist = None;
        let mut album = None;
        let mut genre = None;
        let mut year = None;
        let mut track_number = None;

        // Read metadata tags
        if let Some(metadata) = probed.format.metadata().current() {
            for tag in metadata.tags() {
                if let Some(std_key) = tag.std_key {
                    match std_key {
                        StandardTagKey::TrackTitle => title = Some(tag.value.to_string()),
                        StandardTagKey::Artist => artist = Some(tag.value.to_string()),
                        StandardTagKey::Album => album = Some(tag.value.to_string()),
                        StandardTagKey::Genre => genre = Some(tag.value.to_string()),
                        StandardTagKey::Date => {
                            if let Ok(y) = tag.value.to_string().parse::<u32>() {
                                year = Some(y);
                            }
                        }
                        StandardTagKey::TrackNumber => {
                            if let Ok(n) = tag.value.to_string().parse::<u32>() {
                                track_number = Some(n);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let file_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown Track")
            .to_string();

        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        Ok(Track {
            id: TrackId::new(),
            file_path: path.to_path_buf(),
            title: title.unwrap_or(file_name),
            artist: artist.unwrap_or_else(|| "Unknown Artist".into()),
            album: album.unwrap_or_else(|| "Unknown Album".into()),
            genre,
            year,
            track_number,
            duration_ms,
            bitrate,
            sample_rate,
            channels,
            format: AudioFormat::from_extension(ext),
            replaygain_track_gain: None,
            replaygain_track_peak: None,
            play_count: 0,
            rating: 0,
        })
    }
}
