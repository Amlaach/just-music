#[derive(Debug, Clone, Copy)]
pub struct Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "getrandom patch error")
    }
}

impl std::error::Error for Error {}

pub fn fill(dest: &mut [u8]) -> Result<(), Error> {
    for (i, b) in dest.iter_mut().enumerate() {
        *b = (i % 256) as u8;
    }
    Ok(())
}

pub fn getrandom(dest: &mut [u8]) -> Result<(), Error> {
    fill(dest)
}
