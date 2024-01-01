use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
/// This procedural macro registers a valid function with Antwort's rule engine.
/// Functions must have the signature `fn(&Expr) -> RuleApplicationResult`.
///
/// Intermediary static variables are created to allow for the decentralized registry, with the name `_ANTWORT_GEN_<identifier>`.
/// Care must be taken that these variable names do not conflict with other variables in the scope.
///
/// Below is an example application of this macro:
/// ```
/// #[rule]
/// fn example_rule(_expr: &Expr) -> RuleApplicationResult {
///     Err(RuleApplicationError::RuleNotApplicable)
/// }
///
/// use antwort::rule_engine::get_rules;
/// println!("{:?}", get_rules()); // [Rule { name: "example_rule", application: <fn_pointer> }]
/// ```
/// The function will then be included in Antwort's rule registry, and can be used by the rule engine.
pub fn rule(_: TokenStream, item: TokenStream) -> TokenStream {
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
