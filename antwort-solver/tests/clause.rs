use antwort_solver::ast::Clause;
use antwort_solver::error::Error;

#[test]
fn clause_new() {
    let mut c = Clause::new();
    assert_eq!(c.size(), 0);
    assert!(!c.contains_literal(1));
}

#[test]
fn clause_add_zero() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(0), Err(Error::InvalidLiteral));
    assert_eq!(c.size(), 0);
}

#[test]
fn clause_add_literal() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert!(c.contains_literal(1));
    assert_eq!(c.size(), 1);
}

#[test]
fn clause_add_literals() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert_eq!(c.add_literal(5), Ok(()));
    assert!(c.contains_literal(1));
    assert!(c.contains_literal(5));
    assert_eq!(c.size(), 2);
}

#[test]
fn clause_add_large_literal() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(256), Ok(()));
    assert!(c.contains_literal(256));
    assert_eq!(c.size(), 1);
}

#[test]
fn clause_add_duplicates() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert_eq!(c.add_literal(1), Ok(()));
    assert!(c.contains_literal(1));
    assert_eq!(c.size(), 1);
}

#[test]
fn clause_add_negative() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(-2), Ok(()));
    assert!(c.contains_literal(-2));
    assert_eq!(c.size(), 1);
}

#[test]
fn clause_add_mix() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(-1), Ok(()));
    assert_eq!(c.add_literal(2), Ok(()));
    assert_eq!(c.add_literal(-432), Ok(()));
    assert!(c.contains_literal(-1));
    assert!(c.contains_literal(2));
    assert!(c.contains_literal(-432));
    assert_eq!(c.size(), 3);
}

#[test]
fn clause_remove_literal() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert_eq!(c.remove_literal(1), Ok(()));
    assert!(!c.contains_literal(1));
    assert_eq!(c.size(), 0);
}

#[test]
fn clause_remove_literal_not_present() {
    let mut c = Clause::new();
    assert_eq!(c.remove_literal(1), Ok(()));
    assert!(!c.contains_literal(1));
    assert_eq!(c.size(), 0);
}

#[test]
fn clause_remove_literal_twice() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert_eq!(c.remove_literal(1), Ok(()));
    assert_eq!(c.remove_literal(1), Ok(()));
    assert!(!c.contains_literal(1));
    assert_eq!(c.size(), 0);
}

#[test]
fn clause_remove_literal_negative() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(-1), Ok(()));
    assert_eq!(c.remove_literal(-1), Ok(()));
    assert!(!c.contains_literal(-1));
    assert_eq!(c.size(), 0);
}

#[test]
fn clause_remove_literal_mix() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(-1), Ok(()));
    assert_eq!(c.add_literal(2), Ok(()));
    assert_eq!(c.add_literal(-432), Ok(()));
    assert_eq!(c.remove_literal(-1), Ok(()));
    assert_eq!(c.remove_literal(2), Ok(()));
    assert_eq!(c.remove_literal(-432), Ok(()));
    assert!(!c.contains_literal(-1));
    assert!(!c.contains_literal(2));
    assert!(!c.contains_literal(-432));
    assert_eq!(c.size(), 0);
}
