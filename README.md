<div align="center">

# 🎵 Just Music

**High-Performance Desktop Music Player — Pure Rust**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build](https://github.com/Amlaach/just-music/actions/workflows/ci.yml/badge.svg)](https://github.com/Amlaach/just-music/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Amlaach/just-music?include_prereleases)](https://github.com/Amlaach/just-music/releases)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Platform](https://img.shields.io/badge/Platform-Windows-blue?logo=windows)

</div>

---

## ✨ Features

- **🎧 Multi-Format Audio Playback** — MP3, FLAC, WAV, AAC, OGG Vorbis, Opus, AIFF, ALAC
- **🔒 Lock-Free Audio Pipeline** — Zero-allocation ring buffer for ultra-low-latency playback
- **🎛️ 10-Band Graphic Equalizer** — Biquad IIR filters with perceptual logarithmic volume control
- **🔍 FTS5 Instant Search** — Sub-millisecond full-text search with Unicode support (Hebrew & English)
- **🇮🇱 RTL/BiDi Support** — Full bidirectional text and layout mirroring for Hebrew
- **📊 Multi-Tier Cache** — LRU in-memory + persistent disk cache for artwork and waveforms
- **📁 Drag & Drop** — Drop audio files directly into the player
- **🎨 Dark & Light Themes** — Premium dark-mode UI with smooth theme switching
- **📂 Playlist Management** — Create, manage, and play playlists
- **🕒 Recent History** — Automatically tracks recently played songs
- **⌨️ Keyboard Shortcuts** — Space (play/pause), Ctrl+O (open), Ctrl+, (settings)
- **🔗 File Associations** — Register as default audio player in Windows
- **🧩 WASM Plugin Sandbox** — Isolated WebAssembly plugin execution (Wasmtime)

---

## 🖼️ Screenshots

> Coming soon — the app features a custom frameless window with a sleek dark theme, sidebar navigation, playlist view, and a bottom player bar with playback controls.

---

## 📂 Architecture

```
just-music/
├── Cargo.toml                         # Workspace Configuration
└── crates/
    ├── aether-core/                   # Domain Models, Commands & Events
    ├── aether-audio/                  # Headless Audio Engine (Symphonia + CPAL + DSP)
    ├── aether-storage/                # SQLite Database (WAL) & FTS5 Search Engine
    ├── aether-library/                # Multi-threaded Folder Scanner & File Watcher
    ├── aether-cache/                  # Multi-Tier LRU Memory & Disk Cache
    ├── aether-provider/               # Async Trait Interfaces for Providers
    ├── aether-plugin/                 # Wasmtime WebAssembly Sandbox
    ├── aether-monitor/                # Internal Diagnostics & Metrics
    ├── aether-ui/                     # Design System, BiDi Engine & Views
    └── aether-desktop/                # Binary Entrypoint (just-music.exe)
```

---

## 🛠️ Building from Source

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (Stable 1.75+)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/Amlaach/just-music.git
cd just-music

# Build the desktop app
cargo build --release -p aether-desktop

# Run the desktop app
cargo run -p aether-desktop
```

The compiled binary will be at `target/release/just-music.exe`.

### Running Tests

```bash
cargo test -p aether-core -p aether-audio -p aether-ui -p aether-cache -p aether-monitor
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Space` | Toggle Play / Pause |
| `Ctrl+O` | Open Audio File |
| `Ctrl+,` | Open Settings |

---

## 🔧 Supported Audio Formats

| Format | Extension | Codec |
|--------|-----------|-------|
| MP3 | `.mp3` | MPEG Layer 3 |
| FLAC | `.flac` | Free Lossless Audio Codec |
| WAV | `.wav` | Waveform Audio |
| AAC | `.aac`, `.m4a` | Advanced Audio Coding |
| OGG Vorbis | `.ogg` | Vorbis |
| Opus | `.opus` | Opus Interactive Audio |
| AIFF | `.aiff` | Audio Interchange File Format |
| ALAC | `.alac` | Apple Lossless Audio Codec |

---

## 📦 Downloads

Pre-built binaries are available on the [Releases](https://github.com/Amlaach/just-music/releases) page.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

<div align="center">

**Made with ❤️ and Rust**

</div>
