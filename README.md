<div align="center">

# 🎵 Jost Music (מוזיקה)

**High-Performance Desktop Music Player — Pure Rust**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build](https://github.com/Amlaach/just-music/actions/workflows/ci.yml/badge.svg)](https://github.com/Amlaach/just-music/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Amlaach/just-music?include_prereleases)](https://github.com/Amlaach/just-music/releases)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Platform](https://img.shields.io/badge/Platform-Windows-blue?logo=windows)

</div>

---

## ✨ Overview

**Jost Music** (תת-כותרת: **"מוזיקה"**) is a state-of-the-art, high-performance desktop music application built from the ground up in **Pure Rust**. Featuring a luxury dark crimson & cyan palette, bidirectional text layout (Hebrew RTL default & English LTR), lock-free audio engine, graphic equalizer, and Deep Dark Night Red mode.

---

## 🎨 Design System & Palette

- **Main Dark Background**: `#0a0809` (Deep black with subtle crimson tint)
- **Deep Dark Night Red Mode**: `#050203` (Ultra-dark low-light mode for reduced eye strain)
- **Primary Crimson**: `#8b0000` / `#9b111e` (Luxury maroon cards & header elements)
- **Accent Cyan**: `#00e5ff` (Used for logo **מוזיקה** subtitle, active indicators, visualizer, & hover states)
- **Primary Text**: `#f1f5f9` (Off-white high contrast)
- **Secondary Text**: `#94a3b8` (Muted light grey)

---

## 🌟 Key Features

- **🎧 Multi-Format Audio Playback** — MP3, FLAC, WAV, AAC, OGG Vorbis, Opus, AIFF, ALAC
- **🔒 Lock-Free Audio Pipeline** — Zero-allocation ring buffer for ultra-low-latency playback (Symphonia + CPAL)
- **🇮🇱 RTL/LTR Bidirectional Engine** — Native Hebrew (RTL default) and English (LTR) layout direction switching
- **🌙 Deep Dark Night Red Mode** — Toggleable `#050203` background mode in settings
- **🎛️ Graphic Equalizer** — Multi-band frequency tuning with presets (Flat, Pop, Rock, Bass Boost)
- **🔍 Sub-Millisecond Search** — Sub-millisecond full-text search with Unicode support (Hebrew & English)
- **📊 Multi-Tier Cache** — LRU in-memory + persistent disk cache for artwork and waveforms
- **📁 Drag & Drop** — Drop audio files directly into the player window
- **📂 Playlist Management** — Create, manage, and play custom audio playlists
- **🕒 History & Favorites** — Track recently played tracks and favorite songs
- **⌨️ Keyboard Shortcuts** — `Space` (play/pause), `Ctrl+O` (open file), `Ctrl+,` (settings)
- **🔗 Windows File Associations** — Register as default audio player in Windows

---

## 📂 Architecture

```
just-music/
├── Cargo.toml                         # Workspace Configuration (v1.1.0)
└── crates/
    ├── aether-core/                   # Domain Models, Commands & Events
    ├── aether-audio/                  # Headless Audio Engine (Symphonia + CPAL + DSP)
    ├── aether-storage/                # SQLite Database (WAL) & Search Engine
    ├── aether-library/                # Multi-threaded Folder Scanner & File Watcher
    ├── aether-cache/                  # Multi-Tier LRU Memory & Disk Cache
    ├── aether-provider/               # Async Trait Interfaces for Providers
    ├── aether-plugin/                 # Wasmtime WebAssembly Sandbox
    ├── aether-monitor/                # Internal Diagnostics & Metrics
    ├── aether-ui/                     # Design System, BiDi Engine & Views
    └── aether-desktop/                # Binary Entrypoint (just-music.exe)
```

---

## 🛠️ Building & Testing

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (Stable 1.75+)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/Amlaach/just-music.git
cd just-music

# Build the desktop release app
cargo build --release -p aether-desktop

# Run the application
cargo run -p aether-desktop
```

The compiled binary will be located at `target/release/just-music.exe`.

### Running Automated Tests

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

Pre-built binaries and releases are available on the [Releases](https://github.com/Amlaach/just-music/releases) page.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).

---

<div align="center">

**Made with ❤️ and Rust for Jost Music**

</div>
