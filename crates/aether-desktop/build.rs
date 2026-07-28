fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../assets/icon.ico");
        res.set("ProductName", "Just Music");
        res.set("FileDescription", "Just Music High-Fidelity Audio Player");
        res.set("LegalCopyright", "Copyright (c) 2026 Just Music");
        if let Err(e) = res.compile() {
            eprintln!("Failed to compile Windows resources: {}", e);
        }
    }
}
