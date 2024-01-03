use antwort_core::ast::Expr;

#[test]
fn expr_sub_expressions() {
    let expr = Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    let sub = expr.sub_expressions();
    assert_eq!(sub.len(), 2);
    assert_eq!(sub[0], &Expr::Variable("a".to_string()));
    assert_eq!(sub[1], &Expr::Variable("b".to_string()));
}

#[test]
fn expr_with_sub_expressions() {
    let expr = Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    let sub = expr.sub_expressions();
    let expr2 = expr.with_sub_expressions(sub);
    assert_eq!(expr, expr2);
}

#[test]
fn expr_is_variable() {
    let expr = Expr::Variable("a".to_string());
    assert!(expr.is_variable());
}

#[test]
fn expr_is_negation() {
    let expr = Expr::Negation(Box::new(Expr::Variable("a".to_string())));
    assert!(expr.is_negation());
}

#[test]
fn expr_is_disjunction() {
    let expr = Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(expr.is_disjunction());
}

#[test]
fn expr_is_conjunction() {
    let expr = Expr::Conjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(expr.is_conjunction());
}

#[test]
fn expr_is_implication() {
    let expr = Expr::Implication(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(expr.is_implication());
}

#[test]
fn expr_is_equivalence() {
    let expr = Expr::Equivalence(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(expr.is_equivalence());
}

#[test]
fn expr_is_literal() {
    let expr = Expr::Variable("a".to_string());
    assert!(expr.is_literal());
    let expr = Expr::Negation(Box::new(Expr::Variable("a".to_string())));
    assert!(expr.is_literal());
    let expr = Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(!expr.is_literal());
    let expr = Expr::Conjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(!expr.is_literal());
    let expr = Expr::Implication(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(!expr.is_literal());
    let expr = Expr::Equivalence(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(!expr.is_literal());
}

#[test]
fn expr_is_clause() {
    let expr = Expr::Variable("a".to_string());
    assert!(expr.is_clause());
    let expr = Expr::Negation(Box::new(Expr::Variable("a".to_string())));
    assert!(expr.is_clause());
    let expr = Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(expr.is_clause());
    let expr = Expr::Conjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(!expr.is_clause());
    let expr = Expr::Implication(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(!expr.is_clause());
    let expr = Expr::Equivalence(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(!expr.is_clause());
}

#[test]
fn expr_is_cnf() {
    let expr = Expr::Variable("a".to_string());
    assert!(expr.is_cnf());
    let expr = Expr::Negation(Box::new(Expr::Variable("a".to_string())));
    assert!(expr.is_cnf());
    let expr = Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(expr.is_cnf());
    let expr = Expr::Conjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ]);
    assert!(expr.is_cnf());
    let expr = Expr::Conjunction(vec![Expr::Disjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ])]);
    assert!(expr.is_cnf());
    let expr = Expr::Conjunction(vec![Expr::Conjunction(vec![
        Expr::Variable("a".to_string()),
        Expr::Variable("b".to_string()),
    ])]);
    assert!(!expr.is_cnf());
    let expr = Expr::Implication(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(!expr.is_cnf());
    let expr = Expr::Equivalence(
        Box::new(Expr::Variable("a".to_string())),
        Box::new(Expr::Variable("b".to_string())),
    );
    assert!(!expr.is_cnf());
}
