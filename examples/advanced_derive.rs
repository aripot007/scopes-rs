//! This example shows advanced options you can use to customize how
//! the Scope trait is derived

use std::str::FromStr;

use scopes_rs::{derive::Scope};

#[cfg(feature = "hierarchy")]
use scopes_rs::hierarchy::Hierarchized;


#[derive(Debug, Clone, PartialEq, Scope)]
#[scope(
    // A prefix to add to every scope
    prefix = "myprefix/",

    // Change the separator from the default "." to ":"
    separator = ":",

    // With the `hierarchy` feature, you can set this to false to implement the Hierarchized trait yourself
    // hierarchy = false,
)]
enum ApiScope {
    MultiLevelScope,

    // names are separated at every capital letter, this will be named "scope_with:weird:case"
    #[allow(non_camel_case_types)]
    Scope_withWeirdCase,  

    #[scope(rename = "myscope")]
    MyCustomScope,

    #[scope(rename = "myscope:readonly")]  // With the hierarchy feature, this will be included in MyCustomScope
    MyCustomScopeReadonly,
}

pub fn main() {

    assert_eq!(ApiScope::from_str("myprefix/multi:level:scope").unwrap(), ApiScope::MultiLevelScope);
    assert_eq!(ApiScope::from_str("myprefix/scope_with:weird:case").unwrap(), ApiScope::Scope_withWeirdCase);
    assert_eq!(ApiScope::from_str("myprefix/myscope").unwrap(), ApiScope::MyCustomScope);
    assert_eq!(ApiScope::from_str("myprefix/myscope:readonly").unwrap(), ApiScope::MyCustomScopeReadonly);

    #[cfg(feature = "hierarchy")]
    assert!(ApiScope::MyCustomScope.includes(&ApiScope::MyCustomScopeReadonly))
}
