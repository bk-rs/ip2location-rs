//
pub mod builder;
pub mod querier;

pub use querier::Querier;

//
pub struct PositionRange {
    pub start: u32,
    pub end: u32,
}
impl PositionRange {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

//
#[derive(Debug, Clone, Copy)]
pub enum Category {
    V4,
    V6,
}
