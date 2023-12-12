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

    pub fn find_unit_clause(&self) -> Option<&Clause> {
        self.clauses.iter().find(|c| c.is_unit())
    }

    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    pub fn contains_empty_clause(&self) -> bool {
        self.clauses.iter().any(|c| c.is_empty())
    }
}
