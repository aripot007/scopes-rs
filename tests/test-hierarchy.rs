#![cfg(feature = "hierarchy")]

use scopes_macros::Scope;
use scopes_rs::hierarchy::Hierarchized;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, PartialEq, Scope, EnumIter)]
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

    assert!(MyScope::Bar.includes(&MyScope::BarReadonly));
    assert!(!MyScope::Bar.includes(&MyScope::FooBar));
}

// Check that the generated hierarchy corresponds to the expected one
#[test]
fn test_reference_hierarchy() {

    fn expected_includes(sself: &MyScope, other: &MyScope) -> bool {
        if sself == other { return true }
        match sself {
            MyScope::Foo => matches!(
                other, 
                MyScope::FooBar | MyScope::FooReadonly | MyScope::FooBarReadonly
            ),
            MyScope::FooReadonly => matches!(
                other,
                MyScope::FooBarReadonly
            ),
            MyScope::FooBar => matches!(
                other,
                MyScope::FooBarReadonly
            ),
            MyScope::FooBarReadonly => false,  // No inclusion
            MyScope::Readonly => matches!(
                other,
                MyScope::BarReadonly | MyScope::FooReadonly  // direct inclusion
                | MyScope::FooBarReadonly  // indirect inclusion in FooReadonly 
            ),
            MyScope::Bar => matches!(
                other,
                MyScope::BarReadonly,
            ),
            MyScope::BarReadonly => false,  // No inclusion
        }
    }

    // Test all combinations
    for sself in MyScope::iter() {
        for other in MyScope::iter() {

            assert_eq!(sself.includes(&other), expected_includes(&sself, &other));

        }
    }
}

