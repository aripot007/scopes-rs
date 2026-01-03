use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Scope, hierarchy::inclusion_graph::InclusionGraph};

// Implement the Hierarchized trait
pub fn implement_hierarchized(enum_ident: &Ident, scopes: &HashMap<String, Scope>) -> TokenStream {
    
    // Construct the inclusion graph
    let inclusion_graph = InclusionGraph::from_scopes(scopes.values());

    // Construct the iterators that maps each scope with the ones it includes

    let mut scope_idents: Vec<&Ident> = Vec::new();
    let mut match_patterns: Vec<TokenStream> = Vec::new();

    for scope in inclusion_graph.nodes() {
        
        // If no scope is included in this one, skip it
        if !inclusion_graph.has_neighbors(scope) {
            continue;
        }

        let matched_variants = inclusion_graph.get_included_in(scope);

        // Convert the matched enum variant names to a match pattern :
        // [VariantA, VariantB] becomes `EnumName::VariantA | EnumName::VariantB`
        let match_pattern = quote! {
            #(#enum_ident::#matched_variants)|*
        };

        scope_idents.push(scope);
        match_patterns.push(match_pattern);
    }

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
