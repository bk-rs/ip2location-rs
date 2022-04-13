//
#[derive(Debug)]
pub(super) struct Inner {
    pub(super) bytes: Vec<u8>,
}

impl Inner {
    pub(super) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}
