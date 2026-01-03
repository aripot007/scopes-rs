#![warn(missing_docs)]
//! Macros for [`scopes-rs`](https://github.com/aripot007/scopes-rs)
//!
//! This crate is re-exported by `scopes-rs`, and its documentation should be
//! browsed from the `scopes-rs` crate.
//!  
extern crate proc_macro2;

use darling::FromDeriveInput;
use proc_macro::TokenStream;

use crate::scope::{Scope, opts::ScopeOpts, scope_impl::derive_into_scope_impl};


#[cfg(feature = "hierarchy")]
mod hierarchy;

mod scope;

/// ## Optional `#[scope(...)]` attributes for the enum
/// 
/// - `separator = "..."`: Change the separator between scope labels. Defaults to `"."`
/// - `prefix = "..."`: Add a prefix to every generated scope name. Default is an empty prefix
/// - `hierarchy = bool`: Enable or disable generation of the `Hierarchized` trait. Requires the `hierarchy`
///     feature. Defaults to `true`.
/// - `scope_name_getter = bool`: Implement the `scope_name()` function to get the scope name from a variant (defaults to true)
/// 
/// ## Optional `#[scope(...)]` attributes for enum variants
/// 
/// - `rename = "..."`: Use a specific name instead of inferring it from the variant name
/// - `include = scope | [scope1, ...]`: Include other scopes in the hierarchy. See below for more details.
///     requires the `hierarchy` feature
/// 
/// ### Hierarchy customization
/// 
/// The generated scopes hierarchy can be customized with the `#[scope(include = ...)]` attribute.
/// 
/// The attributes takes a scope variant or a list of scope variants that should be included in the scope :
/// 
/// 
/// ```ignore
/// use scopes_rs::{
///     derive::Scope,
///     hierarchy::Hierarchized,
/// };
/// 
/// #[derive(Clone, Debug, PartialEq, Scope)]
/// enum MyScope {
/// 
///     Foo,
///     #[scope(include = FooBarReadonly)]
///     FooReadonly,
/// 
///     FooBar,
///     FooBarReadonly,
///     
///     #[scope(include = [FooReadonly, BarReadonly])]
///     Readonly,
///     
///     Bar,
///     BarReadonly,
/// }
/// 
/// assert!(MyScope::Readonly.includes(&MyScope::BarReadonly));
/// assert!(MyScope::Readonly.includes(&MyScope::FooReadonly));
/// 
/// // Inclusion is transitive, so Readonly also includes FooBarReadonly 
/// // since FooReadonly includes it
/// assert!(MyScope::Readonly.includes(&MyScope::FooBarReadonly))
/// ```
#[proc_macro_derive(Scope, attributes(scope))]
pub fn derive_into_scope(item: TokenStream) -> TokenStream {
    
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let opts = match ScopeOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(err) => {
            return err.write_errors().into()
        },
    };
    
    derive_into_scope_impl(&opts)
}
