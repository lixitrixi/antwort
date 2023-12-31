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

#[test]
fn formula_remove_clause() {
    let mut f = Formula::new();
    let c1 = Clause::new();
    let c2 = Clause::new();
    f.add_clause(c1);
    f.add_clause(c2);
    f.remove_clause(0);
    assert_eq!(f.size(), 1);
}

#[test]
fn formula_remove_clauses() {
    let mut f = Formula::new();
    let c1 = Clause::new();
    let c2 = Clause::new();
    f.add_clause(c1);
    f.add_clause(c2);
    f.remove_clause(1);
    f.remove_clause(0);
    assert_eq!(f.size(), 0);
}

#[test]
#[should_panic]
fn formula_remove_clause_out_of_bounds() {
    let mut f = Formula::new();
    let c1 = Clause::new();
    let c2 = Clause::new();
    f.add_clause(c1);
    f.add_clause(c2);
    f.remove_clause(2);
    assert_eq!(f.size(), 2);
}

#[test]
#[should_panic]
fn formula_remove_clause_empty() {
    let mut f = Formula::new();
    f.remove_clause(0);
    assert_eq!(f.size(), 0);
}

#[test]
fn formula_is_empty() {
    let mut f = Formula::new();
    assert!(f.is_empty());
    let c = Clause::new();
    f.add_clause(c);
    assert!(!f.is_empty());
}

#[test]
fn formula_contains_empty_clause() {
    let mut f = Formula::new();
    let c = Clause::new();
    f.add_clause(c);
    assert!(f.contains_empty_clause());
}

#[test]
fn formula_contains_empty_clause_empty() {
    let f = Formula::new();
    assert!(!f.contains_empty_clause());
}

#[test]
fn formula_find_unit_clause() {
    let mut f = Formula::new();
    let mut c = Clause::new();
    c.add_literal(1).unwrap();
    f.add_clause(c);
    assert!(f.get_unit_clause().is_some());
}

#[test]
fn formula_find_unit_clause_empty() {
    let f = Formula::new();
    assert!(f.get_unit_clause().is_none());
}

#[test]
fn formula_find_unit_clause_none() {
    let mut f = Formula::new();
    f.add_clause(Clause::new());
    f.add_clause(Clause::new());
    assert!(f.get_unit_clause().is_none());
}
