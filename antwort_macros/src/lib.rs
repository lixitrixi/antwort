use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
/// This procedural macro registers a valid function with Antwort's rule engine.
/// Functions must have the signature `fn(&Expr) -> Result<Expr, RuleApplicationError>`.
///
/// Intermediary static variables are created to allow for the decentralized registry, with the prefix `_ANTWORT_GEN_`.
/// Care must be taken that other variables in the scope do not conflict with these.
///
/// Below is an example application of this macro:
/// ```
/// use antwort::macros::register_rule;
/// use antwort::rule::{RuleApplicationError};
/// use antwort::Expr;
/// #[register_rule]
/// fn example_rule(_expr: &Expr) -> Result<Expr, RuleApplicationError> {
///     Err(RuleApplicationError::RuleNotApplicable)
/// }
///
/// use antwort::rule_engine::get_rules;
/// println!("{:?}", get_rules()); // [Rule { name: "example_rule", application: <fn_pointer> }]
/// ```
pub fn register_rule(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let rule_ident = &func.sig.ident;
    let static_name = format!("_ANTWORT_GEN_RULE_{}", rule_ident).to_uppercase();
    let static_ident = Ident::new(&static_name, rule_ident.span());

    let expanded = quote! {
        #func

        #[::linkme::distributed_slice(::antwort::RULES_DISTRIBUTED_SLICE)]
        static #static_ident: ::antwort::rule::Rule = ::antwort::rule::Rule {
            name: stringify!(#rule_ident),
            application: #rule_ident,
        };
    };

    TokenStream::from(expanded)
}
