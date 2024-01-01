use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
/// This procedural macro registers a valid function with Antwort's rule engine.
/// Functions must have the signature `fn(&Expr) -> RuleApplicationResult`.
///
/// As a step in the decentralized rule registry, an intermediary static variable is created, with the name `ANTWORT_GEN_RULE_<function_name_uppercase>`.
/// Care must be taken that this variable name does not conflict with other variables in the scope.
pub fn rule(_: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: Import necessary modules while avoiding conflicts
    let func = parse_macro_input!(item as ItemFn);
    let rule_ident = &func.sig.ident;
    let static_name = format!("ANTWORT_GEN_RULE_{}", rule_ident).to_uppercase();
    let static_ident = Ident::new(&static_name, rule_ident.span());

    let expanded = quote! {
        use linkme::distributed_slice;

        #func

        #[distributed_slice(RULES_DISTRIBUTED_SLICE)]
        static #static_ident: Rule = Rule {
            application: #rule_ident,
        };
    };

    TokenStream::from(expanded)
}
