use std::env;

pub struct AssociationManager;

impl AssociationManager {
    pub const SUPPORTED_EXTENSIONS: &'static [&'static str] = &[
        "mp3", "flac", "wav", "aac", "ogg", "opus", "m4a", "wma", "aiff",
    ];

    #[cfg(target_os = "windows")]
    pub fn register_all() -> Result<(), String> {
        use winreg::enums::*;
        use winreg::RegKey;

        let exe_path = match env::current_exe() {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(e) => return Err(format!("Failed to get executable path: {e}")),
        };

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let classes = match hkcu.open_subkey_with_flags("Software\\Classes", KEY_WRITE) {
            Ok(key) => key,
            Err(e) => return Err(format!("Failed to open HKCU\\Software\\Classes: {e}")),
        };

        // Create Progression App ID
        let (app_key, _) = classes
            .create_subkey("JustMusic.AudioFile")
            .map_err(|e| format!("Failed to create App ID: {e}"))?;
        app_key.set_value("", &"Just Music Audio File").ok();

        let (icon_key, _) = app_key.create_subkey("DefaultIcon").unwrap();
        icon_key.set_value("", &format!("\"{exe_path}\",0")).ok();

        let (command_key, _) = app_key.create_subkey("shell\\open\\command").unwrap();
        command_key
            .set_value("", &format!("\"{exe_path}\" \"%1\""))
            .ok();

        // Register each extension
        for &ext in Self::SUPPORTED_EXTENSIONS {
            let ext_key_name = format!(".{ext}");
            if let Ok((ext_key, _)) = classes.create_subkey(&ext_key_name) {
                ext_key.set_value("", &"JustMusic.AudioFile").ok();

                let open_with_progids = ext_key.create_subkey("OpenWithProgids").ok();
                if let Some((progids, _)) = open_with_progids {
                    progids.set_value("JustMusic.AudioFile", &"").ok();
                }
            }
        }

        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn register_all() -> Result<(), String> {
        Ok(())
    }

    #[cfg(target_os = "windows")]
    pub fn unregister_all() -> Result<(), String> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(classes) = hkcu.open_subkey_with_flags("Software\\Classes", KEY_WRITE) {
            let _ = classes.delete_subkey_all("JustMusic.AudioFile");
            for &ext in Self::SUPPORTED_EXTENSIONS {
                let ext_key_name = format!(".{ext}");
                if let Ok(ext_key) = classes.open_subkey_with_flags(&ext_key_name, KEY_WRITE) {
                    if let Ok(progids) =
                        ext_key.open_subkey_with_flags("OpenWithProgids", KEY_WRITE)
                    {
                        let _ = progids.delete_value("JustMusic.AudioFile");
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn unregister_all() -> Result<(), String> {
        Ok(())
    }
}
