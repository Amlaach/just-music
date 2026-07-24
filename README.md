# Aether Sound System 🎵

> Commercial-grade, ultra-fast, professional desktop music player written in 100% Pure Rust.

---

## 🌟 Key Features

- **🚀 Ultra-Low Footprint & Lightning Fast Startup:** Sub-30ms cold boot time, running with ~25MB–35MB RAM footprint (up to 10x lighter than CEF/Electron players).
- **🎧 High-Fidelity Audio Engine:** Pure Rust zero-copy audio decoder powered by `Symphonia` supporting MP3, FLAC, WAV, AAC, OGG Vorbis, Opus, AIFF, and ALAC.
- **⚡ Lock-Free Real-Time Streaming:** Hardware output via `CPAL` (WASAPI Exclusive/Shared on Windows) with an SPSC Lock-Free RingBuffer (`rtrb`) for zero allocation in the audio rendering loop.
- **🎛️ 10-Band Graphic Equalizer & DSP:** SIMD-optimized Biquad Direct Form II Transposed IIR filters and perceptual logarithmic volume attenuation with soft peak limiters.
- **🔍 Tantivy Instant Search Engine:** Sub-millisecond prefix and fuzzy full-text search indexing over 500,000+ tracks.
- **🌐 Full Native RTL & LTR Support:** Complete BiDirectional text shaping (`cosmic-text` / `unicode-bidi`) and dynamic layout mirroring for Hebrew and English.
- **📊 Multi-Tier Cache & Virtualization:** LRU Memory & Persistent Disk Cache for album artwork and waveforms, combined with Virtual List recycling for zero-lag scrolling.
- **🧩 WASM Plugin Architecture:** Isolated capability-based WebAssembly sandbox powered by `Wasmtime` for online music and metadata providers.
- **🛡️ 100% Local & Zero Telemetry:** Absolute user privacy with local SQLite storage (WAL mode).

---

## 🏗️ Clean Architecture

The codebase is organized as a Rust workspace with clean layer separation:

```
aether-player/
├── crates/
│   ├── aether-core/        # Domain entities, CQRS PlayerCommands, PlayerEvents & AetherError
│   ├── aether-audio/       # Real-Time Headless Audio Engine (Symphonia, CPAL, RingBuffer, DSP)
│   ├── aether-storage/     # SQLite Database (WAL Mode) & Tantivy FTS Instant Search Engine
│   ├── aether-library/     # Multi-threaded Folder Scanner & Real-Time File Watcher
│   ├── aether-cache/       # Multi-Tier LRU Memory & Persistent Disk Cache Engine
│   ├── aether-provider/    # Async Trait Interfaces for Music & Lyrics Providers
│   ├── aether-plugin/      # Wasmtime WebAssembly Sandbox Engine
│   ├── aether-monitor/     # Internal Local Diagnostics (CPU, RAM, Audio Buffer, Latency)
│   ├── aether-ui/          # Design System, BiDi RTL/LTR Engine & Virtual List Calculator
│   └── aether-desktop/     # Binary Entrypoint & Subsystem Orchestration
```

---

## 🛠️ Building from Source

### Prerequisites
- [Rust](https://rustup.rs/) (Stable 1.75+)

### Build & Run
```bash
# Clone the repository
git clone https://github.com/Amlaach/just-music.git
cd just-music

# Check workspace packages
cargo check --workspace

# Run tests
cargo test --workspace

# Run the player
cargo run -p aether-desktop
```

---

## 📄 License
Licensed under MIT or Apache-2.0.
