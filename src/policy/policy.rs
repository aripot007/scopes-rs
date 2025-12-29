use std::ops::{BitAnd, BitOr, Not};

use crate::scope::Scope;

/// A policy to verify a set of scopes
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

impl<S: Scope> Policy<S> {

    /// Check if a set of scopes is authorized by a policy
    pub fn verify(&self, scopes: &[&S]) -> bool {
        match self {
            
            #[cfg(not(feature = "hierarchy"))]
            Policy::Scope(required) => scopes.contains(&required),

            #[cfg(feature = "hierarchy")]
            Policy::Scope(required) => scopes.iter().any(|s| s.includes(required)),

            Policy::Not(policy) => !policy.verify(scopes),
            Policy::OneOf(policies) => policies.iter().any(|p| p.verify(scopes)),
            Policy::AllOf(policies) => policies.iter().all(|p| p.verify(scopes)),
            Policy::AllowAll => true,
            Policy::DenyAll => false,
        }
    }
}

impl<S: Scope, IntoPolicy: Into<Policy<S>>> BitAnd<IntoPolicy> for Policy<S> {
    type Output = Policy<S>;

    fn bitand(self, rhs: IntoPolicy) -> Self::Output {
        let rhs: Policy<S> = rhs.into();
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

impl<S: Scope, IntoPolicy: Into<Policy<S>>> BitOr<IntoPolicy> for Policy<S> {
    type Output = Policy<S>;

    fn bitor(self, rhs: IntoPolicy) -> Self::Output {
        let rhs: Policy<S> = rhs.into();
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

impl<S: Scope> Not for Policy<S> {
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

impl<S: Scope> From<S> for Policy<S> {
    fn from(value: S) -> Self {
        Policy::Scope(value)
    }
}

pub trait IntoPolicy<S: Scope> {
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
