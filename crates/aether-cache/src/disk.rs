use aether_core::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct DiskCache {
    base_dir: PathBuf,
}

impl DiskCache {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self> {
        let path = base_dir.as_ref().to_path_buf();
        fs::create_dir_all(&path)?;
        Ok(Self { base_dir: path })
    }

    pub fn get_bytes(&self, key: &str) -> Option<Vec<u8>> {
        let file_path = self.base_dir.join(key);
        fs::read(file_path).ok()
    }

    pub fn put_bytes(&self, key: &str, data: &[u8]) -> Result<()> {
        let file_path = self.base_dir.join(key);
        fs::write(file_path, data)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir)?;
            fs::create_dir_all(&self.base_dir)?;
        }
        Ok(())
    }
}
