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

#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    use crate::{Scope, hierarchy::is_included};

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

    #[test]
    fn test_self_inclusion() {
        let scope_single_label = new_scope("ScopeA", &["scope"]);
        let scope_multiple_labels = new_scope("ScopeB", &["scope", "foo", "bar"]);

        assert!(is_included(&scope_single_label, &scope_single_label));
        assert!(is_included(&scope_multiple_labels, &scope_multiple_labels));
    }

    #[test]
    fn test_inclusion_simple() {
        let scope_foo = new_scope("Foo", &["foo"]);
        let scope_foo_bar = new_scope("Foobar", &["foo", "bar"]);

        assert!(is_included(&scope_foo_bar, &scope_foo));
        assert!(!is_included(&scope_foo, &scope_foo_bar));
    }

    #[test]
    fn test_inclusion_transitive() {
        let scope_foo = new_scope("Foo", &["foo"]);
        let scope_foo_bar = new_scope("FooBar", &["foo", "bar"]);
        let scope_foo_bar_baz = new_scope("FooBarBaz", &["foo", "bar", "baz"]);

        assert!(is_included(&scope_foo_bar, &scope_foo));
        assert!(is_included(&scope_foo_bar_baz, &scope_foo_bar));
        assert!(is_included(&scope_foo_bar_baz, &scope_foo))
    }

    #[test]
    fn test_non_inclusion_label_prefix() {
        let scope_foo = new_scope("Foo", &["foo"]);
        let scope_foobar = new_scope("Foobar", &["foobar"]);

        assert!(!is_included(&scope_foobar, &scope_foo))
    }

    #[test]
    fn test_non_inclusion() {
        let scope_foo_bar = new_scope("Foo", &["foo", "bar"]);
        let scope_bar = new_scope("Bar", &["bar"]);
        let scope_foo_bar_baz = new_scope("FooBarBaz", &["foo", "bar", "baz"]);
        let scope_foo_baz_baz = new_scope("FooBazBaz", &["foo", "baz", "baz"]);

        assert!(is_included(&scope_foo_bar_baz, &scope_foo_bar));
        assert!(!is_included(&scope_foo_baz_baz, &scope_foo_bar));
        assert!(!is_included(&scope_foo_bar, &scope_bar));
        assert!(!is_included(&scope_bar, &scope_foo_bar));
    }
}
