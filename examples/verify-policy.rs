use std::str::FromStr;

use scopes_rs::{policy::{IntoPolicy, PolicyBuilder}, derive::Scope};


#[derive(Clone, PartialEq, Scope)]
#[scope(prefix = "https://myapi.test/")]  // The prefix is optional, it will be added to each scope
enum ApiScope {
    Admin,
    Profile,
    ProfileReadonly,
    Contacts,
    ContactsReadonly,

    #[scope(rename = "myscope")]
    MyCustomScope,
}


pub fn main() {

    // Parse the scopes from a web request for example
    let scopes = parse_scopes("https://myapi.test/profile https://myapi.test/contacts.readonly");

    // Check if a policy accepts the given set of scopes

    let admin_policy = ApiScope::Admin.into_policy();

    assert_eq!(false, admin_policy.verify(&scopes));


    // You can use the PolicyBuilder to build complex policies
    let complex_policy = PolicyBuilder::new()
        .require(ApiScope::Profile)
        .require(ApiScope::ContactsReadonly)
        .and(
            PolicyBuilder::not(ApiScope::MyCustomScope)
        )
        .build();

    assert_eq!(true, complex_policy.verify(&scopes));

    let more_scopes = [ApiScope::Profile, ApiScope::ContactsReadonly, ApiScope::MyCustomScope];
    assert_eq!(false, complex_policy.verify(more_scopes));


    // Policies can also be combined together with &, | and !
    
    let complex_or_admin = complex_policy | (admin_policy & !ApiScope::MyCustomScope.into_policy());

    assert_eq!(true, complex_or_admin.verify([ApiScope::Admin]));


    // With the `hierarchy` feature, scopes can include other ones, and will validate the policy
    #[cfg(feature = "hierarchy")]
    {
        let readonly_policy = ApiScope::ContactsReadonly.into_policy();

        // "contacts" includes "contacts.readonly"
        assert_eq!(true, readonly_policy.verify([ApiScope::Contacts]));
    }

}

// You would usually parse the scopes from a request in a middleware with
// proper error handling, but this is sufficient here
fn parse_scopes(data: &str) -> Vec<ApiScope> {

    data.split_whitespace()
        .filter_map(|s| ApiScope::from_str(s).ok())
        .collect()
}
