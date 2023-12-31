use std::ops::Index;

use crate::{Clause, Literal};

#[derive(Clone, Debug, PartialEq)]
pub struct Formula {
    pub clauses: Vec<Clause>,
}

impl Index<usize> for Formula {
    type Output = Clause;

    fn index(&self, index: usize) -> &Clause {
        &self.clauses[index]
    }
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

    /// Returns the index and value of a unit clause in this formula, if one exists.
    pub fn get_unit_clause(&self) -> Option<(usize, &Clause)> {
        self.clauses.iter().enumerate().find(|(_, c)| c.is_unit())
    }

    /// Returns `true` if this formula contains no clauses.
    pub fn is_empty(&self) -> bool {
        self.clauses.is_empty()
    }

    /// Returns `true` if this formula contains a clause with no literals.
    pub fn contains_empty_clause(&self) -> bool {
        self.clauses.iter().any(|c| c.is_empty())
    }

    /// Simplifies this formula, assuming the given literal is true.
    /// Namely, removes all clauses containing the literal and removes the negated literal from all other clauses.
    pub fn simplify(&mut self, literal: Literal) {
        self.clauses.retain(|c| !c.contains_literal(literal));
        for clause in self.clauses.iter_mut() {
            clause.remove_literal(-literal);
        }
    }
}
