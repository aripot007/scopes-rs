#![cfg(all(test, feature = "hierarchy"))]

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
    let mut scope_baz = new_scope("Baz", &["baz"]);
    scope_baz.include.push(scope_foo_bar.ident.clone());

    let g = InclusionGraph::from_scopes([&scope_foo, &scope_foo_bar, &scope_foo_bar_baz, &scope_baz].into_iter());

    // Check label inclusion
    assert!(is_included(&g, &scope_foo_bar, &scope_foo));
    assert!(is_included(&g, &scope_foo_bar_baz, &scope_foo_bar));
    assert!(is_included(&g, &scope_foo_bar_baz, &scope_foo));

    // Check inclusion with include list
    assert!(is_included(&g, &scope_foo_bar, &scope_baz));
    assert!(is_included(&g, &scope_foo_bar_baz, &scope_baz));
    assert!(!is_included(&g, &scope_foo, &scope_baz));
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