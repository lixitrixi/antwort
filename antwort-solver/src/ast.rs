use crate::error::{Error, Result};

pub struct Clause {
    pos_literals: Vec<u64>, // Bit arrays of positive literals
    neg_literals: Vec<u64>, // Each "slice" can hold 64 literals
    size: usize,            // The number of literals in this clause
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
    pub fn size(&self) -> usize {
        self.size
    }

    /// Adds a literal to this clause.
    pub fn add_literal(&mut self, literal: i32) -> Result<()> {
        let dest = self.get_sign_vector(literal)?;
        let (i, j) = get_index_offset(literal);

        if i >= dest.len() {
            // Grow the vector to fit the index
            dest.resize(i + 1, 0);
        }
        if dest[i] & (1 << j) != 0 {
            // Literal already exists
            return Ok(());
        }
        dest[i] |= 1 << j;
        self.size += 1;
        Ok(())
    }

    /// Returns true if this clause contains the given literal.
    pub fn contains_literal(&mut self, literal: i32) -> bool {
        let src = self.get_sign_vector(literal);
        let src = match src {
            Ok(src) => src,
            Err(_) => return false,
        };
        let (i, j) = get_index_offset(literal);

        if i >= src.len() {
            // Vector does not reach the literal index
            return false;
        }
        src[i] & (1 << j) != 0
    }

    pub fn remove_literal(&mut self, literal: i32) -> Result<()> {
        let dest = self.get_sign_vector(literal)?;
        let (i, j) = get_index_offset(literal);

        if i >= dest.len() || dest[i] & (1 << j) == 0 {
            // Vector does not reach the literal index
            // or literal is not present
            return Ok(());
        }
        dest[i] &= !(1 << j);
        self.size -= 1;
        Ok(())
    }

    fn get_sign_vector(&mut self, literal: i32) -> Result<&mut Vec<u64>> {
        match literal.signum() {
            1 => Ok(&mut self.pos_literals),
            -1 => Ok(&mut self.neg_literals),
            _ => Err(Error::InvalidLiteral),
        }
    }
}

/// Returns the index and offset of a literal in a vector of bit arrays.
fn get_index_offset(literal: i32) -> (usize, usize) {
    let literal = literal.abs() - 1; // From 1- to 0-indexed
    let i = literal / 64;
    let j = literal % 64;
    (i as usize, j as usize)
}
