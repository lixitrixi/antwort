use crate::error::{Error, Result};

pub struct Clause {
    pos_literals: Vec<u64>, // Bit arrays of positive literals
    neg_literals: Vec<u64>, // Each "slice" can hold 64 literals
    size: i32,              // The number of literals in this clause
}

impl Clause {
    pub fn new() -> Clause {
        Clause {
            pos_literals: Vec::new(),
            neg_literals: Vec::new(),
            size: 0,
        }
    }

    /// The number of literals in this clause.
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Adds a literal to this clause.
    pub fn add_literal(&mut self, literal: i32) -> Result<()> {
        let dest = match literal.signum() {
            1 => &mut self.pos_literals,
            -1 => &mut self.neg_literals,
            _ => return Err(Error::InvalidLiteral),
        };
        let literal = literal.abs() - 1; // From 1- to 0-indexed

        let i = literal / 64;
        let j = literal % 64;
        if i as usize >= dest.len() {
            // Grow the vector if necessary
            dest.resize((i + 1) as usize, 0);
        }
        dest[i as usize] |= 1 << j;
        Ok(()) // TODO: increase size
    }

    pub fn contains_literal(&self, literal: i32) -> bool {
        false
    }
}
