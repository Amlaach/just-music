use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-l" && i + 1 < args.len() {
            let lib_path = &args[i + 1];
            if let Some(parent) = std::path::Path::new(lib_path).parent() {
                let _ = fs::create_dir_all(parent);
            }
            // Valid COFF archive header magic bytes
            let _ = fs::write(lib_path, b"!<arch>\n");
            i += 2;
            continue;
        }
        i += 1;
    }
}
