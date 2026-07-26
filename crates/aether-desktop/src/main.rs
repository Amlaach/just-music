#![windows_subsystem = "windows"]

use aether_audio::HeadlessAudioEngine;
use aether_ui::JustMusicApp;
use eframe::NativeOptions;
use egui::ViewportBuilder;
use std::path::PathBuf;

fn main() {
    // Set up panic handler FIRST - logs panic to a file so we can debug
    // even though windows_subsystem = "windows" hides the console.
    let log_dir = dirs_for_log();
    let panic_log = log_dir.join("just-music-panic.log");
    let panic_log_clone = panic_log.clone();
    std::panic::set_hook(Box::new(move |info| {
        let msg = format!(
            "[{}] PANIC: {}\nBacktrace:\n{:?}\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            info,
            std::backtrace::Backtrace::capture()
        );
        let _ = std::fs::write(&panic_log_clone, &msg);
        // Also try to show a message box on Windows
        #[cfg(target_os = "windows")]
        {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;
            let wide_msg: Vec<u16> = OsStr::new(&format!(
                "Just Music encountered an error and needs to close.\n\nDetails saved to:\n{}\n\nError: {}",
                panic_log_clone.display(),
                info
            ))
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
            let wide_title: Vec<u16> = OsStr::new("Just Music - Error")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            unsafe {
                windows_sys::Win32::UI::WindowsAndMessaging::MessageBoxW(
                    std::ptr::null_mut(),
                    wide_msg.as_ptr(),
                    wide_title.as_ptr(),
                    0x00000010, // MB_ICONERROR
                );
            }
        }
    }));

    // Initialize logging to file
    let log_file = log_dir.join("just-music.log");
    let file_appender = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .ok();

    if let Some(writer) = file_appender {
        tracing_subscriber::fmt()
            .with_writer(std::sync::Mutex::new(writer))
            .with_ansi(false)
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    tracing::info!("Just Music v1.0.0 starting...");

    // Initialize Headless Audio Engine (Headless backend thread)
    // Audio engine failure is NOT fatal - the UI still works for browsing
    let audio_handle = match HeadlessAudioEngine::spawn() {
        Ok(handle) => {
            tracing::info!("Audio engine initialized successfully");
            Some(handle)
        }
        Err(e) => {
            tracing::error!("Audio engine failed to initialize: {}. Continuing without audio.", e);
            None
        }
    };

    // Check CLI argument for opening a track on startup (e.g. Open With from Explorer)
    let mut initial_track = None;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let file_arg = args[1..].join(" ");
        let raw_path = file_arg.trim().trim_matches('"').trim_matches('\'');
        let file_path = PathBuf::from(raw_path);
        if file_path.exists() {
            tracing::info!("Opening file from CLI: {}", file_path.display());
            initial_track = Some(file_path);
        }
    }

    let viewport = ViewportBuilder::default()
        .with_title("Just Music")
        .with_decorations(false)
        .with_inner_size([920.0, 620.0])
        .with_min_inner_size([800.0, 520.0]);

    let native_options = NativeOptions {
        viewport,
        ..Default::default()
    };

    tracing::info!("Launching GUI...");

    if let Err(e) = eframe::run_native(
        "Just Music",
        native_options,
        Box::new(move |cc| {
            let mut app = JustMusicApp::new(cc, audio_handle);
            if let Some(path) = initial_track {
                aether_ui::views::home_view::load_file(
                    path,
                    &mut app.state,
                    app.audio_handle.as_ref(),
                );
            }
            Ok(Box::new(app))
        }),
    ) {
        tracing::error!("eframe::run_native failed: {}", e);
        let error_msg = format!("Just Music failed to start: {}", e);
        let _ = std::fs::write(&panic_log, &error_msg);
    }
}

fn dirs_for_log() -> PathBuf {
    let dir = if let Some(data_dir) = dirs_next::data_local_dir() {
        data_dir.join("JustMusic")
    } else {
        PathBuf::from(".")
    };
    let _ = std::fs::create_dir_all(&dir);
    dir
}
