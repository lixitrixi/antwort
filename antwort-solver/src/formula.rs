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

    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    pub fn size(&self) -> usize {
        self.clauses.len()
    }
}
