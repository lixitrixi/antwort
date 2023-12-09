use crate::error::{Error, Result};

pub struct Clause {
    pos_literals: Vec<u64>,
    neg_literals: Vec<u64>,
    size: usize,
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            pos_literals: Vec::new(),
            neg_literals: Vec::new(),
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add_literal(&mut self, literal: i32) -> Result<()> {
        Err(Error::InvalidLiteral)
    }

    pub fn contains_literal(&self, literal: i32) -> bool {
        false
    }
}
