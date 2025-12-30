use std::str::FromStr;

use scopes_macros::Scope;
use scopes_rs::{error::ScopeParseError, policy::{IntoPolicy, Policy}};

#[cfg(feature = "hierarchy")]
use scopes_rs::hierarchy::Hierarchized;

#[derive(Debug, PartialEq, Scope)]
enum MyScope {
    Foo,
    FooBar,
    Baz,
    Bar,
}

#[test]
fn test_parsing() {

    assert_eq!(MyScope::Foo, MyScope::from_str("foo").expect("Could not parse scope"));
    assert_eq!(MyScope::FooBar, MyScope::from_str("foo.bar").expect("Could not parse scope"));
    assert_eq!(MyScope::Baz, MyScope::from_str("baz").expect("Could not parse scope"));

}

#[test]
fn test_invalid_parsing() {
    assert!(MyScope::from_str("not_a_scope").is_err());
    assert!(MyScope::from_str("foobar").is_err());
    assert!(MyScope::from_str("").is_err());

    // Check that generated implementation uses the correct error
    match MyScope::from_str("") {
        Ok(_) => panic!("Parsing an empty string should return an error"),
        Err(ScopeParseError(_)) => (),
    }
}

#[test]
fn test_simple_policy() {
    let single_scope = MyScope::Foo.into_policy();

    assert!(single_scope.verify(&[MyScope::Foo]));
    assert!(!single_scope.verify(&[MyScope::FooBar]));

    let accept_all = Policy::<MyScope>::AllowAll;

    assert!(accept_all.verify(Vec::<&MyScope>::new()));

    let reject_all = !accept_all;
    
    assert!(!reject_all.verify(Vec::<&MyScope>::new()));
    assert!(!reject_all.verify(&[MyScope::Foo, MyScope::FooBar, MyScope::Baz, MyScope::Bar]));
}

#[test]
fn test_complex_policy() {

    let foobar_or_bar_and_baz = 
        MyScope::FooBar.into_policy() 
        | (MyScope::Bar.into_policy() & MyScope::Baz.into_policy());
    
    assert!(foobar_or_bar_and_baz.verify(&[MyScope::FooBar]));
    assert!(foobar_or_bar_and_baz.verify(&[MyScope::Bar, MyScope::Baz]));
    assert!(foobar_or_bar_and_baz.verify(&[MyScope::Bar, MyScope::FooBar, MyScope::Baz]));
    assert!(!foobar_or_bar_and_baz.verify(&[MyScope::Bar]));
    assert!(!foobar_or_bar_and_baz.verify(&[MyScope::Baz]));

    #[cfg(feature = "hierarchy")]
    assert!(foobar_or_bar_and_baz.verify(&[MyScope::Foo]));

    #[cfg(not(feature = "hierarchy"))]
    assert!(!foobar_or_bar_and_baz.verify(&[MyScope::Foo]));

    let not_baz = !MyScope::Baz.into_policy();

    assert!(not_baz.verify(Vec::<&MyScope>::new()));
    assert!(not_baz.verify(&[MyScope::Foo]));
    assert!(!not_baz.verify(&[MyScope::Baz]));

}

#[test]
#[cfg(feature = "hierarchy")]
fn test_hierarchy() {

    assert!(MyScope::Foo.includes(&MyScope::Foo));
    assert!(MyScope::Foo.includes(&MyScope::FooBar));
    assert!(!MyScope::Foo.includes(&MyScope::Bar));
    assert!(!MyScope::Foo.includes(&MyScope::Baz));

    assert!(!MyScope::FooBar.includes(&MyScope::Foo));
    assert!(MyScope::FooBar.includes(&MyScope::FooBar));
    assert!(!MyScope::FooBar.includes(&MyScope::Bar));
    assert!(!MyScope::FooBar.includes(&MyScope::Baz));

    assert!(!MyScope::Bar.includes(&MyScope::Foo));
    assert!(!MyScope::Bar.includes(&MyScope::FooBar));
    assert!(MyScope::Bar.includes(&MyScope::Bar));
    assert!(!MyScope::Bar.includes(&MyScope::Baz));

    assert!(!MyScope::Baz.includes(&MyScope::Foo));
    assert!(!MyScope::Baz.includes(&MyScope::FooBar));
    assert!(!MyScope::Baz.includes(&MyScope::Bar));
    assert!(MyScope::Baz.includes(&MyScope::Baz));

}

