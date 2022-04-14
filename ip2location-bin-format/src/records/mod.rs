//
pub mod builder;
pub mod querier;

pub use querier::{V4Querier, V6Querier};

//
#[derive(Debug, Clone, Copy, Default)]
pub struct PositionRange {
    pub start: u32,
    pub end: u32,
}
impl PositionRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}
