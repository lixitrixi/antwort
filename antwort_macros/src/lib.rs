use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn};

#[proc_macro_attribute]
/// This procedural macro registers a decorated function with Antwort's rule engine.
/// Functions must have the signature `fn(&Expr) -> Result<Expr, RuleApplicationError>`.
///
/// Intermediary static variables are created to allow for the decentralized registry, with the prefix `_ANTWORT_GEN_`.
/// Please ensure that other variable names do not conflict with these.
pub fn register_rule(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let rule_ident = &func.sig.ident;
    let static_name = format!("_ANTWORT_GEN_RULE_{}", rule_ident).to_uppercase();
    let static_ident = Ident::new(&static_name, rule_ident.span());

    let expanded = quote! {
        #func

        #[::linkme::distributed_slice(::antwort::_RULES_DISTRIBUTED_SLICE)]
        static #static_ident: ::antwort::rule::Rule = ::antwort::rule::Rule {
            name: stringify!(#rule_ident),
            application: #rule_ident,
        };
    };

    TokenStream::from(expanded)
}
