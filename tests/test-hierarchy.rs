#![cfg(feature = "hierarchy")]

use scopes_macros::Scope;
use scopes_rs::hierarchy::Hierarchized;

#[derive(Clone, Debug, PartialEq, Scope)]
enum MyScope {

    Foo,

    #[scope(include = FooBarReadonly)]
    FooReadonly,

    FooBar,

    FooBarReadonly,
    
    #[scope(include = [FooReadonly, BarReadonly])]
    Readonly,
    
    Bar,
    BarReadonly,
}

#[test]
fn test_manual_include() {
    assert!(MyScope::Readonly.includes(&MyScope::FooReadonly));
    assert!(MyScope::Readonly.includes(&MyScope::BarReadonly));

    // Test transitivity
    // FooReadonly includes FooBarReadonly so Readonly should also include it
    assert!(MyScope::Readonly.includes(&MyScope::FooBarReadonly));
}

#[test]
fn test_self_inclusion() {
    assert!(MyScope::Foo.includes(&MyScope::Foo));
    assert!(MyScope::Foo.included_in(&MyScope::Foo));
}

#[test]
fn test_label_include() {
    assert!(MyScope::Foo.includes(&MyScope::FooBar));
    assert!(MyScope::Foo.includes(&MyScope::FooReadonly));
    assert!(MyScope::Foo.includes(&MyScope::FooBarReadonly));
}

