use antwort_solver::ast::Clause;

#[test]
fn clause_new() {
    let c = Clause::new();
    assert_eq!(c.size(), 0);
}

#[test]
fn clause_add_literal() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert!(c.contains_literal(1));
}

#[test]
fn clause_add_duplicates() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert_eq!(c.add_literal(1), Ok(()));
    assert!(c.contains_literal(1));
}

#[test]
fn clause_add_negative() {
    let mut c = Clause::new();
    assert_eq!(c.add_literal(1), Ok(()));
    assert_eq!(c.add_literal(-2), Ok(()));
    assert!(c.contains_literal(1));
    assert!(c.contains_literal(-2));
}
