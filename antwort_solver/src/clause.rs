use crate::{Error, Literal, Result};

#[derive(Clone, Debug, PartialEq)]
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

    /// Adds a literal to this clause. A negative literal represents its negated form.
    pub fn add_literal(&mut self, literal: Literal) -> Result<()> {
        let dest: &mut Vec<u64> = self.get_sign_vector_mut(literal)?;
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
    /// # Returns
    /// `false` if the literal is not present or is invalid.
    pub fn contains_literal(&self, literal: Literal) -> bool {
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

    /// Removes the given literal from this clause.
    /// # Returns
    /// `Ok(())` if the literal was removed.
    pub fn remove_literal(&mut self, literal: Literal) -> Result<()> {
        let dest = self.get_sign_vector_mut(literal)?;
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

    /// Returns an arbitrary literal present in this clause.
    /// This method iterates over the underlying bit array and is therefore linear time.
    /// If there is no literal in this clause, this method returns `None`.
    pub fn get_literal(&self) -> Option<Literal> {
        for (i, &word) in self.pos_literals.iter().enumerate() {
            if word != 0 {
                let literal = (word.trailing_zeros() as usize) + (i * 64);
                return Some((literal + 1) as Literal); // From 0- to 1-indexed
            }
        }
        for (i, &word) in self.neg_literals.iter().enumerate() {
            if word != 0 {
                let literal = (word.trailing_zeros() as usize) + (i * 64);
                return Some(-((literal + 1) as Literal)); // From 0- to 1-indexed
            }
        }
        return None;
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_unit(&self) -> bool {
        self.size == 1
    }

    fn get_sign_vector(&self, literal: Literal) -> Result<&Vec<u64>> {
        match literal.signum() {
            1 => Ok(&self.pos_literals),
            -1 => Ok(&self.neg_literals),
            _ => Err(Error::InvalidLiteral),
        }
    }

    fn get_sign_vector_mut(&mut self, literal: Literal) -> Result<&mut Vec<u64>> {
        match literal.signum() {
            1 => Ok(&mut self.pos_literals),
            -1 => Ok(&mut self.neg_literals),
            _ => Err(Error::InvalidLiteral),
        }
    }
}

/// Returns the index and offset of a literal in a vector of bit arrays.
fn get_index_offset(literal: Literal) -> (usize, usize) {
    let literal = literal.abs() - 1; // From 1- to 0-indexed
    let i = literal / 64;
    let j = literal % 64;
    (i as usize, j as usize)
}
