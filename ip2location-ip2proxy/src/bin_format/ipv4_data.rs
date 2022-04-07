use core::fmt;

use tokio::fs::File as TokioFile;

//
pub struct Ipv4Data {
    file: TokioFile,
}

impl fmt::Debug for Ipv4Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ipv4Data").finish()
    }
}

impl Ipv4Data {
    pub fn new(file: TokioFile) -> Self {
        Self { file }
    }
}
