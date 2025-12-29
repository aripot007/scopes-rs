use std::str::FromStr;

#[cfg(feature = "hierarchy")]
use crate::hierarchy::Hierarchized;


#[cfg(not(feature = "hierarchy"))]
pub trait Scope: FromStr + PartialEq {}

#[cfg(feature = "hierarchy")]
pub trait Scope: FromStr + PartialEq + Hierarchized {}

pub trait AsScopeRef<S: ?Sized> {
    fn as_scope_ref(&self) -> &S;
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
