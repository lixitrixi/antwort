use antwort_solver::{solve, Clause, Error, Formula};

#[test]
fn solve_empty() {
    let f = Formula::new();
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 0);
}

#[test]
fn solve_empty_clause() {
    let mut f = Formula::new();
    let c = Clause::new();
    f.add_clause(c);
    let s = solve(&f);
    assert_eq!(s, Err(Error::Unsatisfiable));
}

#[test]
fn solve_single_literal() {
    let mut f = Formula::new();
    let mut c = Clause::new();
    c.add_literal(1);
    f.add_clause(c);
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 1);
    assert_eq!(s.assignments[0], 1);
}

#[test]
fn solve_single_literal_negated() {
    let mut f = Formula::new();
    let mut c = Clause::new();
    c.add_literal(-1);
    f.add_clause(c);
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 1);
    assert_eq!(s.assignments[0], -1);
}

#[test]
fn solve_single_clause() {
    let mut f = Formula::new();
    let mut c = Clause::new();
    let literals = vec![1, 2, 3];
    for l in literals.as_slice() {
        c.add_literal(*l);
    }
    f.add_clause(c);
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 1);
    assert!(literals.contains(&s.assignments[0]));
}

#[test]
fn solve_single_clause_negated() {
    let mut f = Formula::new();
    let mut c = Clause::new();
    let literals = vec![-1, -2, -3];
    for l in literals.as_slice() {
        c.add_literal(*l);
    }
    f.add_clause(c);
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 1);
    assert!(literals.contains(&s.assignments[0]));
}

#[test]
fn solve_two_clauses() {
    let mut f = Formula::new();
    let mut c1 = Clause::new();
    let mut c2 = Clause::new();
    let literals1 = vec![1, 2, 3];
    let literals2 = vec![4, 5, 6];
    for l in literals1.as_slice() {
        c1.add_literal(*l);
    }
    for l in literals2.as_slice() {
        c2.add_literal(*l);
    }
    f.add_clause(c1);
    f.add_clause(c2);
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 2);
    assert!(literals1.contains(&s.assignments[0]) || literals2.contains(&s.assignments[0]));
    assert!(literals1.contains(&s.assignments[1]) || literals2.contains(&s.assignments[1]));
}

#[test]
fn solve_two_clauses_negated() {
    let mut f = Formula::new();
    let mut c1 = Clause::new();
    let mut c2 = Clause::new();
    let literals1 = vec![-1, -2, -3];
    let literals2 = vec![-4, -5, -6];
    for l in literals1.as_slice() {
        c1.add_literal(*l);
    }
    for l in literals2.as_slice() {
        c2.add_literal(*l);
    }
    f.add_clause(c1);
    f.add_clause(c2);
    let s = solve(&f).unwrap();
    assert_eq!(s.assignments.len(), 2);
    assert!(literals1.contains(&s.assignments[0]) || literals2.contains(&s.assignments[0]));
    assert!(literals1.contains(&s.assignments[1]) || literals2.contains(&s.assignments[1]));
}
