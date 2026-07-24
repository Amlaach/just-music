<div align="center">

# 🎵 Aether Sound System (`just-music`)

**High-Performance Audio Engine & Desktop Music Player Core in Pure Rust**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/Status-In_Active_Development_(WIP)-orange)](#-project-status)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Platform](https://img.shields.io/badge/Platform-Windows-blue?logo=windows)

</div>

---

> [!WARNING]
> ### 🚧 Project Status: Active Development (Work In Progress)
> **`just-music` / Aether Sound System is currently under active pre-alpha development.**  
> The core headless audio engine, SQLite database schema, instant search, file scanner/watcher, multi-tier cache, and BiDi layout engine are implemented in Pure Rust. The graphical UI renderer is actively being wired up.

---

## 🌟 Architecture & Implemented Core Subsystems

- **🎧 Real-Time Audio Engine (`aether-audio`):** Pure Rust zero-copy audio decoder powered by `Symphonia` (supporting MP3, FLAC, WAV, AAC, OGG Vorbis, Opus, AIFF, ALAC) with low-latency `CPAL` WASAPI hardware output.
- **🔒 Lock-Free Audio Pipeline:** Single-Producer Single-Consumer (`rtrb`) lock-free ring buffer for zero allocation in the audio playback loop.
- **🎛️ 10-Band Graphic Equalizer & Volume Limiter:** Biquad Direct Form II Transposed IIR filters and perceptual logarithmic volume attenuation with soft peak limiters.
- **🔍 SQLite FTS5 Instant Search (`aether-storage`):** Sub-millisecond full-text search indexing over audio tracks with unicode support for Hebrew and English.
- **🇮🇱 BiDi Text & Mirroring Engine (`aether-ui`):** BiDirectional text direction analysis (`unicode-bidi`) and dynamic layout mirroring for Hebrew (RTL) and English (LTR).
- **📊 Multi-Tier Cache (`aether-cache`):** LRU In-Memory and persistent disk cache manager for artwork, thumbnails, and waveforms.
- **📁 Multi-Threaded Library Scanner (`aether-library`):** Parallel directory traversal (`walkdir`) and real-time filesystem change watcher (`notify`).
- **🧩 WASM Plugin Sandbox (`aether-plugin`):** Isolated WebAssembly plugin execution environment (`Wasmtime`).

---

## 📂 Workspace Structure

```
aether-player/
├── LICENSE                        # MIT License
├── Cargo.toml                     # Workspace Configuration
└── crates/
    ├── aether-core/               # Domain Models, CQRS Commands & PlayerEvents
    ├── aether-audio/              # Headless Audio Engine (Symphonia, CPAL, RingBuffer, DSP)
    ├── aether-storage/            # SQLite Database (WAL Mode) & FTS5 Instant Search Engine
    ├── aether-library/            # Multi-threaded Folder Scanner & Real-Time File Watcher
    ├── aether-cache/              # Multi-Tier LRU Memory & Persistent Disk Cache Engine
    ├── aether-provider/           # Async Trait Interfaces for Music & Lyrics Providers
    ├── aether-plugin/             # Wasmtime WebAssembly Sandbox Engine
    ├── aether-monitor/            # Internal Diagnostic Metrics (CPU, RAM, Audio Buffer)
    ├── aether-ui/                 # Design System, BiDi RTL/LTR Engine & Virtual List
    └── aether-desktop/            # Binary Entrypoint
```

---

## 🛠️ Building & Running Locally

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (Stable 1.75+)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/Amlaach/just-music.git
cd just-music

# Check workspace crates
cargo check --workspace

# Run core unit tests
cargo test -p aether-core -p aether-audio -p aether-ui -p aether-cache -p aether-monitor

# Run desktop entrypoint
cargo run -p aether-desktop
```

---

## 📄 License
This project is licensed under the [MIT License](LICENSE).
