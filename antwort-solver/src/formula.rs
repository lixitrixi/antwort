use crate::Clause;

pub struct Formula {
    clauses: Vec<Clause>,
}

impl Formula {
    pub fn new() -> Formula {
        Formula {
            clauses: Vec::new(),
        }
    }

    /// Adds a clause to the formula.
    /// # Panics
    /// This method may panic if the new number of clauses exceeds `isize::MAX`
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    /// Removes and returns the clause at the given index.
    /// Order of clauses is not maintained when removing.
    /// # Panics
    /// This method may panic if the index is out of bounds.
    pub fn remove_clause(&mut self, index: usize) {
        self.clauses.swap_remove(index);
    }

    /// Returns the number of clauses in this formula.
    pub fn size(&self) -> usize {
        self.clauses.len()
    }
}
