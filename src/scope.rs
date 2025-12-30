//! Contains the trait that types representing a scope should implement

use std::str::FromStr;

#[cfg(feature = "hierarchy")]
use crate::hierarchy::Hierarchized;
use crate::policy::{Policy, PolicyBuilder};

#[cfg(not(feature = "hierarchy"))]
/// A trait implemented by types representing a scope.
/// 
/// The [`FromStr`] implementation is used to parse scopes
/// from strings.
pub trait Scope: FromStr + PartialEq {}

#[cfg(feature = "hierarchy")]
/// A trait implemented by types representing a scope.
/// 
/// The [`FromStr`] implementation is used to parse scopes
/// from strings.
/// 
/// The [`Hierarchized`] trait is only required with the `hierarchy`
/// feature.
pub trait Scope: FromStr + PartialEq + Hierarchized {}


/// Used to do a cheap reference-to-reference conversion
pub trait AsScopeRef<S: Scope> {
    /// Converts this type to a reference 
    fn as_scope_ref(&self) -> &S;
}

impl<S: Scope> Policy<S> {
    /// Create a new [`PolicyBuilder<S>`]
    /// 
    /// Equivalent to calling [`PolicyBuilder<S>::new`]
    pub fn builder() -> PolicyBuilder<S> {
        PolicyBuilder::new()
    }
}

impl<S: Scope> AsScopeRef<S> for S {
    fn as_scope_ref(&self) -> &S {
        self
    }
}

impl<'a, S: Scope> AsScopeRef<S> for &'a S {
    fn as_scope_ref(&self) -> &S {
        *self
    }
}

impl<'a, 'b, S: Scope> AsScopeRef<S> for &'a &'b S {
    fn as_scope_ref(&self) -> &S {
        **self
    }
}

#[cfg(test)]
mod tests {
    use crate::scope::AsScopeRef;

    #[test]
    fn test_as_scope_ref() {

        let scope = "foo".to_string();

        assert_eq!(&scope, scope.as_scope_ref());
        assert_eq!(&scope, (&scope).as_scope_ref());
        assert_eq!(&scope, (&&scope).as_scope_ref());
        assert_eq!(&scope, (&&&scope).as_scope_ref());

    }
}
