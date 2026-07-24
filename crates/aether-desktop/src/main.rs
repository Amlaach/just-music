use aether_audio::HeadlessAudioEngine;
use aether_core::{PlayState, PlayerCommand, PlayerEvent};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().or_else(|_| EnvFilter::try_new("info")))
        .init();

    tracing::info!("Starting Aether Sound System (Production Desktop Core)");

    // Initialize Headless Audio Engine
    let audio_handle = HeadlessAudioEngine::spawn()?;
    tracing::info!("Headless Real-Time Audio Engine spawned successfully");

    // Initialize subsystems
    aether_storage::storage_init();
    aether_library::library_init();
    aether_cache::cache_init();
    aether_ui::ui_init();

    // Verify engine responsiveness
    audio_handle.send_command(PlayerCommand::SetVolume(0.85))?;
    audio_handle.send_command(PlayerCommand::SetEqualizerEnabled(true))?;

    tracing::info!("Aether Sound System initialization complete. Engine running cleanly.");
    Ok(())
}
