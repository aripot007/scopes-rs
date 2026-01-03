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

#[cfg(all(test, feature = "hierarchy"))]
mod tests {
    use proc_macro2::Span;

    use crate::{Scope, hierarchy::{inclusion_graph::InclusionGraph}};

    // Create a new scope with the given identifier name and labels.
    // The separator is "." and the prefix is empty.
    fn new_scope(ident_name: &str, labels: &[&str]) -> Scope {
        Scope::_test_new(
            syn::Ident::new(ident_name, Span::call_site()),
            labels.into_iter(),
            ".",
            "",
        )
    }

    // Check if scope a is included in scope b
    fn is_included(g: &InclusionGraph, scope_a: &Scope, scope_b: &Scope) -> bool {
        g.get_included_in(&scope_b.ident).find(|i| &scope_a.ident == *i).is_some()
    }

    #[test]
    fn test_inclusion_simple() {
        let scope_foo = new_scope("Foo", &["foo"]);
        let scope_foo_bar = new_scope("Foobar", &["foo", "bar"]);

        let g = InclusionGraph::from_scopes([&scope_foo, &scope_foo_bar].into_iter());

        assert!(is_included(&g, &scope_foo_bar, &scope_foo));
        assert!(!is_included(&g, &scope_foo, &scope_foo_bar));
    }

    #[test]
    fn test_inclusion_transitive() {
        let scope_foo = new_scope("Foo", &["foo"]);
        let scope_foo_bar = new_scope("FooBar", &["foo", "bar"]);
        let scope_foo_bar_baz = new_scope("FooBarBaz", &["foo", "bar", "baz"]);

        let g = InclusionGraph::from_scopes([&scope_foo, &scope_foo_bar, &scope_foo_bar_baz].into_iter());

        assert!(is_included(&g, &scope_foo_bar, &scope_foo));
        assert!(is_included(&g, &scope_foo_bar_baz, &scope_foo_bar));
        assert!(is_included(&g, &scope_foo_bar_baz, &scope_foo))
    }

    #[test]
    fn test_non_inclusion_label_prefix() {
        let scope_foo = new_scope("Foo", &["foo"]);
        let scope_foobar = new_scope("Foobar", &["foobar"]);

        let g = InclusionGraph::from_scopes([&scope_foo, &scope_foobar].into_iter());

        assert!(!is_included(&g, &scope_foobar, &scope_foo))
    }

    #[test]
    fn test_non_inclusion() {
        let scope_foo_bar = new_scope("Foo", &["foo", "bar"]);
        let scope_bar = new_scope("Bar", &["bar"]);
        let scope_foo_bar_baz = new_scope("FooBarBaz", &["foo", "bar", "baz"]);
        let scope_foo_baz_baz = new_scope("FooBazBaz", &["foo", "baz", "baz"]);

        let g = InclusionGraph::from_scopes([&scope_foo_bar, &scope_bar, &scope_foo_bar_baz, &scope_foo_baz_baz].into_iter());

        assert!(is_included(&g, &scope_foo_bar_baz, &scope_foo_bar));
        assert!(!is_included(&g, &scope_foo_baz_baz, &scope_foo_bar));
        assert!(!is_included(&g, &scope_foo_bar, &scope_bar));
        assert!(!is_included(&g, &scope_bar, &scope_foo_bar));
    }
}
