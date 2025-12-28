use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::Scope;

// Check if scope_1 is included in scope_2
// A scope includes another one if its label list is a prefix of the other's label list
fn is_included(scope_1: &Scope, scope_2: &Scope) -> bool {
    scope_1.labels.starts_with(&scope_2.labels)
}

// Implement the Hierarchized trait
pub fn implement_hierarchized(enum_ident: &Ident, scopes: &HashMap<String, Scope>) -> TokenStream {
    
    // Ideally, we could construct a tree for better flexibility, but for the first poc
    // comparing each label list is sufficient

    // Construct the iterators that maps each scope with the ones it includes

    let mut scope_idents: Vec<Ident> = Vec::new();
    let mut match_patterns: Vec<TokenStream> = Vec::new();

    for scope in scopes.values() {
        
        let matched_variants: Vec<Ident> = scopes.values().filter_map(|other_scope| {

            if other_scope != scope && is_included(other_scope, scope) {
                Some(other_scope.ident.clone())
            } else {
                None
            }
        }).collect();

        // If no scope is included in this one, skip it
        if matched_variants.len() == 0 {
            continue;
        }

        // Convert the matched enum variant names to a match pattern :
        // [VariantA, VariantB] becomes `EnumName::VariantA | EnumName::VariantB`
        let match_pattern = quote! {
            #(#enum_ident::#matched_variants)|*
        };

        scope_idents.push(scope.ident.clone());
        match_patterns.push(match_pattern);
    }

    let scope_idents = scope_idents.into_iter();
    let match_patterns = match_patterns.into_iter();

    quote! {
        impl ::scopes_rs::hierarchy::Hierarchized for #enum_ident {
            fn includes(&self, other: &Self) -> bool {
                if self == other {
                    return true;
                }

                match self {
                    #(#enum_ident::#scope_idents => matches!(other, #match_patterns),)*
                    _ => false
                }
            }
        }

    }.into()
}
