use antwort_solver::{Clause, Error, Formula};

#[test]
fn formula_new() {
    let f = Formula::new();
    assert_eq!(f.size(), 0);
}

#[test]
fn formula_add_clause() {
    let mut f = Formula::new();
    let c = Clause::new();
    f.add_clause(c);
    assert_eq!(f.size(), 1);
}

#[test]
fn formula_add_clauses() {
    let mut f = Formula::new();
    let c1 = Clause::new();
    let c2 = Clause::new();
    f.add_clause(c1);
    f.add_clause(c2);
    assert_eq!(f.size(), 2);
}
