use std::str::FromStr;

use scopes_macros::Scope;
use scopes_rs::{hierarchy::Hierarchized, policy::IntoPolicy};

#[derive(Debug, PartialEq, Scope)]
#[scope(prefix = "myprefix/", separator = "_")]
enum MyScope {
    Foo,
    FooBar,
    Bar,
    // Should not be included in MyScope::Bar
    #[scope(rename = "barstool")]
    BarStool,
    // Should be included in MyScope::Foo with hierarchy feature
    #[scope(rename = "foo_baz")]
    RenameSeparated,
}

#[test]
fn test_parsing() {
    assert_eq!(MyScope::Foo, MyScope::from_str("myprefix/foo").expect("Could not parse scope"));
    assert_eq!(MyScope::FooBar, MyScope::from_str("myprefix/foo_bar").expect("Could not parse scope"));
    assert_eq!(MyScope::Bar, MyScope::from_str("myprefix/bar").expect("Could not parse scope"));
    assert_eq!(MyScope::BarStool, MyScope::from_str("myprefix/barstool").expect("Could not parse scope"));
    assert_eq!(MyScope::RenameSeparated, MyScope::from_str("myprefix/foo_baz").expect("Could not parse scope"));

    // Parsing without a prefix should not work
    assert!(MyScope::from_str("foo").is_err());

    // Renamed field should not match the generated name
    assert!(MyScope::from_str("bar_stool").is_err());
    assert!(MyScope::from_str("myprefix/bar_stool").is_err());
    assert!(MyScope::from_str("rename_separated").is_err());
    assert!(MyScope::from_str("myprefix/rename_separated").is_err());

}

#[test]
#[cfg_attr(not(feature = "hierarchy"), ignore = "This test requires the `hierarchy` feature")]
fn test_hierarchy() {

    assert!(MyScope::Foo.includes(&MyScope::FooBar));
    assert!(MyScope::Foo.includes(&MyScope::RenameSeparated));
    assert!(!MyScope::Bar.includes(&MyScope::BarStool));
   
    let policy = MyScope::RenameSeparated.into_policy();
    
    assert!(policy.verify(&[&MyScope::RenameSeparated]));
    assert!(policy.verify(&[&MyScope::Foo]));
    assert!(!policy.verify(&[&MyScope::FooBar]));
    assert!(!policy.verify(&[&MyScope::Bar]));
}

