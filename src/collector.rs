use std::ops::Range;

struct Item {
    range: Range<usize>,
    data: String,
    items: Vec<Item>,
}

pub struct Collector {
    root: Item,
}

impl Collector {
    pub fn new() -> Self {
        Self { root }
    }
}
