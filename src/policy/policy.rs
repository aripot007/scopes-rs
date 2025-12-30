use std::ops::{BitAnd, BitOr, Not};

use crate::scope::{AsScopeRef, Scope};

/// A policy to verify a set of scopes
/// 
/// Policies can be combined using the `&`, `|` and `!` operators :
/// 
/// ```
/// # use scopes_rs::derive::Scope;
/// # use scopes_rs::policy::IntoPolicy;
/// # #[derive(Clone, PartialEq, Scope)]
/// # enum MyScope {Foo, Bar, Baz}
/// let policy_a = MyScope::Foo.into_policy() & MyScope::Bar.into_policy();
/// 
/// let policy_b = (MyScope::Baz.into_policy() | MyScope::Bar.into_policy()) & !policy_a;
/// ```
/// 
/// You can also use a [`PolicyBuilder<S>`](crate::policy::PolicyBuilder) to build
/// complex policies.
/// 
#[derive(PartialEq)]
#[cfg_attr(any(test, feature = "debug"), derive(Debug))]
pub enum Policy<S: Scope> {

    /// Requires a scope to be present
    Scope(S),

    /// Requires one of the policies to be verified
    OneOf(Vec<Policy<S>>),

    /// Requires all of the policies to be verified
    AllOf(Vec<Policy<S>>),

    /// Requires a policy not to be verified
    Not(Box<Policy<S>>),

    /// Policy that always accept everything
    AllowAll,

    /// Policy that accepts nothing
    DenyAll,
}

impl<S> Clone for Policy<S>
where S: Scope + Clone {
    fn clone(&self) -> Self {
        match self {
            Self::Scope(arg0) => Self::Scope(arg0.clone()),
            Self::OneOf(arg0) => Self::OneOf(arg0.clone()),
            Self::AllOf(arg0) => Self::AllOf(arg0.clone()),
            Self::Not(arg0) => Self::Not(arg0.clone()),
            Self::AllowAll => Self::AllowAll,
            Self::DenyAll => Self::DenyAll,
        }
    }
}

impl<S> Policy<S> where S: Scope {

    /// Check if a set of scopes is authorized by a policy
    pub fn verify<Iterator>(&self, scopes: Iterator) -> bool 
    where 
        Iterator: IntoIterator + Clone,
        Iterator::Item: AsScopeRef<S>,
    {
        match self {
            
            #[cfg(not(feature = "hierarchy"))]
            Policy::Scope(required) => scopes.into_iter().find(|s| s.as_scope_ref() == required).is_some(),

            #[cfg(feature = "hierarchy")]
            Policy::Scope(required) => scopes.into_iter().any(|s| s.as_scope_ref().includes(required)),

            Policy::Not(policy) => !policy.verify(scopes),
            Policy::OneOf(policies) => policies.iter().any(|p| p.verify(scopes.clone())),
            Policy::AllOf(policies) => policies.iter().all(|p| p.verify(scopes.clone())),
            Policy::AllowAll => true,
            Policy::DenyAll => false,
        }
    }
}

impl<S, I> BitAnd<I> for Policy<S>
where 
    S: Scope,
    I: IntoPolicy<S>,
{
    type Output = Policy<S>;

    fn bitand(self, rhs: I) -> Self::Output {
        let rhs: Policy<S> = rhs.into_policy();
        match (self, rhs) {
            // If the 2 policies are AllOf, merge them
            (Policy::AllOf(mut left), Policy::AllOf(mut right)) => {

                left.append(&mut right);
                Policy::AllOf(left)
            },

            // If one of the policy is DenyAll, return DenyAll as it would always fail anyway
            (Policy::DenyAll, _) | (_, Policy::DenyAll) => Policy::DenyAll,

            // If one of them is already AllOf, add the other to it
            (Policy::AllOf(mut policies), other)
            | (other, Policy::AllOf(mut policies)) => {

                policies.push(other);
                Policy::AllOf(policies)
            }

            (left, right) => Policy::AllOf(vec![left, right])
        }
    }
}

impl<S, I> BitOr<I> for Policy<S>
where 
    S: Scope,
    I: IntoPolicy<S>,
{
    type Output = Policy<S>;

    fn bitor(self, rhs: I) -> Self::Output {
        let rhs: Policy<S> = rhs.into_policy();
        match (self, rhs) {
            // If the 2 policies are OneOf, merge them
            (Policy::OneOf(mut left), Policy::OneOf(mut right)) => {

                left.append(&mut right);
                Policy::OneOf(left)
            },

            // If one of the policy is AllowAll, return AllowAll as it would always evaluate to true
            (Policy::AllowAll, _) | (_, Policy::AllowAll) => Policy::AllowAll,

            // If one of them is already OneOf, add the other to it
            (Policy::OneOf(mut policies), other)
            | (other, Policy::OneOf(mut policies)) => {

                policies.push(other);
                Policy::OneOf(policies)
            }

            (left, right) => Policy::OneOf(vec![left, right])
        }
    }
}

impl<S> Not for Policy<S>
where 
    S: Scope,
{
    type Output = Policy<S>;

    fn not(self) -> Self::Output {
        match self {
            Policy::AllowAll => Policy::DenyAll,
            Policy::DenyAll => Policy::AllowAll,
            Policy::Not(policy) => *policy,
            p => Policy::Not(Box::new(p)),
        }
    }
}

impl<S> From<S> for Policy<S>
where
    S: Scope
{
    fn from(value: S) -> Self {
        Policy::Scope(value)
    }
}

impl<S> From<&S> for Policy<S>
where
    S: Scope + Clone
{
    fn from(value: &S) -> Self {
        Policy::Scope(value.clone())
    }
}

/// A trait implemented by a type that can be converted into a [`Policy<S>`].
/// 
/// This is implemented for anything that implements [`Scope`], and 
/// allows passing them directly to a [`PolicyBuilder<S>`](crate::policy::PolicyBuilder)
/// or on the right of `&`, `|` and `!` operators when combining policies.
/// 
/// ```
/// # use scopes_rs::derive::Scope;
/// # #[derive(Clone, PartialEq, Scope)]
/// # enum MyScope {Foo, Bar, Baz}
/// use scopes_rs::policy::{IntoPolicy, PolicyBuilder};
/// 
/// let policy = PolicyBuilder::not(MyScope::Bar).build();
/// let policy = policy | MyScope::Baz;
/// ```
pub trait IntoPolicy<S> where S: Scope {

    /// Converts this type to a [`Policy<S>`]
    fn into_policy(self) -> Policy<S>;
}

impl<S, I> IntoPolicy<S> for I
where 
    S: Scope,
    I: Into<Policy<S>>,
{
    fn into_policy(self) -> Policy<S> {
        self.into()
    }
}
