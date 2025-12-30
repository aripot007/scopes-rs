#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
//!
//! # Crate features
//! 
//! - `hierarchy`: Enable hierarchical scopes support. See [`hierarchy`] for more details.
//! 

pub mod scope;

pub mod error;

#[cfg(feature = "hierarchy")]
pub mod hierarchy;

pub mod policy;

/// Derive macro for the [`Scope`](scope::Scope) trait
pub mod derive {

    /// Derive macro to implement the [`Scope`] trait
    /// 
    /// Derives an implementation of [`Scope`] for an enum. The enum should also
    /// implement [`PartialEq`].
    /// 
    /// It infers the scope name from the enum variant for the [`FromStr`](std::str::FromStr) implementation,
    /// and for ordering scopes for the [`Hierarchized`] trait (when the `hierarchy` feature is enabled).
    /// 
    /// 
    /// ```
    /// use scopes_rs::derive::Scope;
    /// 
    /// #[derive(Clone, PartialEq, Scope)]
    /// enum MyScope {
    ///     Foo,
    ///     Bar,
    /// }
    /// ```
    /// 
    /// # Inferred scope name
    /// 
    /// The scope name is inferred from the enum variant name by splitting it at each uppercase letter,
    /// the converting each label to lowercase, and joining the labels with the `separator` (`.` by default).
    /// 
    /// A few examples :
    /// 
    /// | Variant name | Inferred name |
    /// |--------------|---------------|
    /// | `Foo` | `foo` |
    /// | `FooBar` | `foo.bar` |
    /// | `foo_bar` | `foo_bar` |
    /// | `BAZ` | `b.a.z` |
    /// 
    /// Labels do not convoy any special meaning, but are used to infer the scopes hierarchy 
    /// (when the `hierarchy` feature is enabled). A scope will include another one if its
    /// labels are a prefix of the other one. For example, `foo.bar` includes `foo.bar.baz` but
    /// not `baz`. See the [`hierarchy`](crate::hierarchy) module documentation for more details.
    /// 
    ///  When using the `rename` attribute, the labels will be parsed by splitting the name by `separator`.
    /// 
    /// # Errors
    /// 
    /// Compilation will fail if scopes have conflicting names :
    /// 
    /// ```compile_fail
    /// #[derive(Clone, PartialEq, Scope)]
    /// enum MyScope {
    ///     Foo,
    ///     #[scope(rename = "foo")]
    ///     Bar,
    /// }
    /// ```
    /// 
    /// # Optional attributes 
    /// 
    /// [`Scope`]: ../scope/Scope
    /// [`Hierarchized`]: ../hierarchy/Hierarchized
    pub use scopes_macros::Scope;
}

// Implement the `Scope` trait for String for the tests
#[cfg(test)]
mod tests {
    use crate::scope::Scope;

    #[cfg(feature = "hierarchy")]
    use crate::hierarchy::Hierarchized;

    impl Scope for String {}

    #[cfg(feature = "hierarchy")]
    impl Hierarchized for String {
        fn includes(&self, other: &Self) -> bool {
            if self == other {
                return true;
            }
            other.starts_with(self)
        }
    }

}
