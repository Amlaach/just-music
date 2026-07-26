use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

static COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "getrandom patch error")
    }
}

impl std::error::Error for Error {}

pub fn fill(dest: &mut [u8]) -> Result<(), Error> {
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(123456789);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut state = nanos.wrapping_add(count).wrapping_mul(6364136223846793005);
    for b in dest.iter_mut() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        *b = (state >> 32) as u8;
    }
    Ok(())
}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    fill(dest)
}
