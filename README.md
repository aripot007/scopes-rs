# Scopes-rs

`scopes-rs` is a strongly typed scoped authorization library.

# Features

- Verify complex scope authorization policies
- Manipulate scopes in a strongly-typed fashion
- Generate boilerplate implementation with a derive macro
- Support for hierarchized scopes with the `hierarchy` feature

# Usage example

```rust
use std::str::FromStr;
use scopes_rs::{
    policy::IntoPolicy,
    derive::Scope
};


#[derive(Clone, PartialEq, Scope)]
enum ApiScope {
    Settings,
    SettingsReadonly,
    Profile,
    ProfileReadonly
}


pub fn main() {

    // Parse the scopes from a web request for example
    let scopes: Vec<ApiScope> = parse_scopes("profile settings.readonly");

    // Convert a single scope in a policy that requires this scope
    let policy = ApiScope::SettingsReadonly.into_policy();

    assert_eq!(true, policy.verify(&scopes));

    // Policies can also be combined together with &, | and !
    let other_policy = policy & ApiScope::ProfileReadonly;

    // With the hierarchy feature, "profile.readonly" is included in "profile", so this is accepted
    #[cfg(feature = "hierarchy")]
    assert_eq!(true, other_policy.verify(&scopes));

    // Otherwise, the scopes require an exact match
    #[cfg(not(feature = "hierarchy"))]
    assert_eq!(false, other_policy.verify(&scopes));
}

// You would usually parse the scopes from a request in a middleware with
// proper error handling, but this is sufficient here
fn parse_scopes(data: &str) -> Vec<ApiScope> {

    data.split_whitespace()
        .filter_map(|s| ApiScope::from_str(s).ok())
        .collect()
}
```

The [`examples`](https://github.com/aripot007/scopes-rs/tree/master/examples) directory
contains some other examples of how to use this library.

The docs also provide more code snippets and examples.

# License

This project is dual-licensed under the [Apache 2.0 License](https://github.com/aripot007/scopes-rs/blob/master/LICENSE-APACHE) and the [MIT License](https://github.com/aripot007/scopes-rs/blob/master/LICENSE-MIT).
You can choose between either of these if you want to use this work.

`SPDX-License-Identifier: Apache-2.0 OR MIT`
