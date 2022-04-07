use core::fmt;

use tokio::fs::File as TokioFile;

//
pub struct Ipv6Data {
    file: TokioFile,
}

impl fmt::Debug for Ipv6Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ipv6Data").finish()
    }
}

impl Ipv6Data {
    pub fn new(file: TokioFile) -> Self {
        Self { file }
    }
}
