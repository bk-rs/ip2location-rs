use crate::index::INDEX_LEN;

//
pub struct Builder {
    bytes: Vec<u8>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(INDEX_LEN as usize),
        }
    }

    pub fn append(&mut self, slice: &[u8]) {
        self.bytes.extend_from_slice(slice);
    }

    pub fn finish<T>(self) -> Result<T, BuildError>
    where
        T: From<Vec<u8>>,
    {
        if self.bytes.len() != INDEX_LEN as usize {
            return Err(BuildError::LenMismatch);
        }

        Ok(T::from(self.bytes))
    }
}

//
#[derive(Debug)]
pub enum BuildError {
    LenMismatch,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for BuildError {}
