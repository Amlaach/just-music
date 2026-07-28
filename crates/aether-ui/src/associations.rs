use std::env;

pub struct AssociationManager;

impl AssociationManager {
    pub const SUPPORTED_EXTENSIONS: &'static [&'static str] = &[
        "mp3", "flac", "wav", "ogg", "m4a", "aac", "opus", "wma", "aiff",
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

        // ProgID registration
        let app_prog_id = "JustMusic.AudioFile";
        let (app_key, _) = classes
            .create_subkey(app_prog_id)
            .map_err(|e| format!("Failed to create ProgID key: {e}"))?;
        let _ = app_key.set_value("", &"Just Music Audio File");
        let _ = app_key.set_value("FriendlyTypeName", &"Just Music Audio File");

        if let Ok((icon_key, _)) = app_key.create_subkey("DefaultIcon") {
            let _ = icon_key.set_value("", &format!("\"{exe_path}\",0"));
        }

        if let Ok((command_key, _)) = app_key.create_subkey("shell\\open\\command") {
            let _ = command_key.set_value("", &format!("\"{exe_path}\" \"%1\""));
        }

        // Register each file extension with OpenWithProgids & ProgID
        for &ext in Self::SUPPORTED_EXTENSIONS {
            let ext_key_name = format!(".{ext}");
            if let Ok((ext_key, _)) = classes.create_subkey(&ext_key_name) {
                let _ = ext_key.set_value("", &app_prog_id);

                if let Ok((progids, _)) = ext_key.create_subkey("OpenWithProgids") {
                    let _ = progids.set_value(app_prog_id, &"");
                }
            }
        }

        // Capabilities & RegisteredApplications registration
        if let Ok(software) = hkcu.open_subkey_with_flags("Software", KEY_WRITE) {
            if let Ok((cap_key, _)) = software.create_subkey("JustMusic\\Capabilities") {
                let _ = cap_key.set_value("ApplicationName", &"Just Music");
                let _ = cap_key.set_value(
                    "ApplicationDescription",
                    &"Just Music High-Fidelity Audio Player",
                );
                if let Ok((assoc_key, _)) = cap_key.create_subkey("FileAssociations") {
                    for &ext in Self::SUPPORTED_EXTENSIONS {
                        let _ = assoc_key.set_value(format!(".{ext}"), &app_prog_id);
                    }
                }
            }

            if let Ok((reg_apps, _)) = software.create_subkey("RegisteredApplications") {
                let _ = reg_apps.set_value("JustMusic", &"Software\\JustMusic\\Capabilities");
            }
        }

        // Notify Windows Shell of file association changes
        Self::notify_shell_change();

        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn notify_shell_change() {
        use windows_sys::Win32::UI::Shell::{SHChangeNotify, SHCNE_ASSOCCHANGED, SHCNF_FLUSH};
        unsafe {
            SHChangeNotify(
                SHCNE_ASSOCCHANGED as i32,
                SHCNF_FLUSH,
                std::ptr::null(),
                std::ptr::null(),
            );
        }
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
        if let Ok(software) = hkcu.open_subkey_with_flags("Software", KEY_WRITE) {
            let _ = software.delete_subkey_all("JustMusic");
            if let Ok(reg_apps) =
                software.open_subkey_with_flags("RegisteredApplications", KEY_WRITE)
            {
                let _ = reg_apps.delete_value("JustMusic");
            }
        }
        Self::notify_shell_change();
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn unregister_all() -> Result<(), String> {
        Ok(())
    }
}
