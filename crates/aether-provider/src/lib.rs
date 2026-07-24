use aether_core::{Result, TrackId};
use async_trait::async_trait;

#[async_trait]
pub trait MusicProvider: Send + Sync {
    fn provider_id(&self) -> &'static str;
    async fn search(&self, query: &str) -> Result<Vec<TrackId>>;
}

#[async_trait]
pub trait LyricsProvider: Send + Sync {
    async fn fetch_lyrics(&self, artist: &str, title: &str) -> Result<Option<String>>;
}
