use crate::{policy::{IntoPolicy, Policy}, scope::Scope};

/// A builder to help create complex policies
/// 
/// ```
/// # use scopes_rs::derive::Scope;
/// # #[derive(Clone, PartialEq, Scope)]
/// # enum MyScope {Foo, Bar, Baz}
/// use scopes_rs::policy::PolicyBuilder;
/// 
/// let complex_policy = PolicyBuilder::new()
///     .require(MyScope::Foo)
///     .require(MyScope::Bar)
///     .or(
///        PolicyBuilder::not(MyScope::Foo).and(MyScope::Baz) 
///     )
///     .build();
/// 
/// assert!(complex_policy.verify([MyScope::Foo, MyScope::Bar]));
/// assert!(complex_policy.verify([MyScope::Bar, MyScope::Baz]));
/// assert!(!complex_policy.verify([MyScope::Foo, MyScope::Baz]));
/// ```
/// 
#[cfg_attr(test, derive(Debug, Clone, PartialEq))]
pub struct PolicyBuilder<S: Scope>(Option<Policy<S>>);

impl<S: Scope> PolicyBuilder<S> {

    /// Create a new PolicyBuilder.
    /// 
    /// The default policy produced by an unmodified builder is a policy that rejects everything.
    pub fn new() -> Self {
        Self(None)
    }

    /// Create a builder from a policy
    pub fn from_policy(policy: impl IntoPolicy<S>) -> Self {
        Self(Some(policy.into_policy()))
    }

    /// Build the policy
    pub fn build(self) -> Policy<S> {
        self.0.unwrap_or(Policy::DenyAll)
    }

    // Constructors

    /// Creates a builder that rejects a policy
    /// 
    /// ```
    /// # use scopes_rs::derive::Scope;
    /// use scopes_rs::policy::*;
    /// # #[derive(PartialEq, Debug, Scope)] enum MyScope {Foo, Bar}
    /// 
    /// let policy = PolicyBuilder::not(MyScope::Foo).build();
    /// 
    /// assert!(!policy.verify(&[&MyScope::Foo]));
    /// assert!(policy.verify(&[&MyScope::Bar]));
    /// ```
    pub fn not(policy: impl IntoPolicy<S>) -> Self {
        Self(Some(!policy.into_policy()))
    }

    /// Creates a builder that requires one of the given policies or scopes
    /// 
    /// This is syntactic sugar for `PolicyBuilder::new().require_any(...)`
    pub fn one_of(policies: impl IntoIterator<Item = impl IntoPolicy<S>>) -> Self {
        Self::new().require_any(policies)
    }

    /// Creates a builder that requires all of the given policies or scopes
    /// 
    /// This is syntactic sugar for `PolicyBuilder::new().require_all(...)`
    pub fn all_of(policies: impl IntoIterator<Item = impl IntoPolicy<S>>) -> Self {
        Self::new().require_all(policies)
    }

    /// Add a required scope to the policy
    /// 
    /// This is equivalent to `policy & scope`.
    pub fn require(self, scope: S) -> Self {
        let policy = match self.0 {
            Some(p) => p & scope,
            None => scope.into_policy(),
        };
        Self(Some(policy))
    }

    /// Add multiple required scopes or policies to the policy
    /// 
    /// This is equivalent to `policy & Policy::AllOf(scopes)`.
    /// 
    /// This function can take a mix of scopes or policies :
    /// ```ignore
    /// let policy: Policy = ... ; 
    /// 
    /// PolicyBuilder::new()
    ///     .require_all(&[MyScope::Foo, &policy, MyScope::Bar])
    /// ```
    pub fn require_all(self, scopes: impl IntoIterator<Item = impl IntoPolicy<S>>) -> Self {
        let all = Policy::AllOf(scopes.into_iter().map(IntoPolicy::into_policy).collect());
        
        let policy = match self.0 {
            Some(p) => p & all,
            None => all,
        };

        Self(Some(policy))
    }

    /// Require any of the given scopes or policies
    /// 
    /// This is equivalent to `policy & Policy::OneOf(scopes)`.
    /// 
    /// This function can take a mix of scopes or policies :
    /// ```ignore
    /// let policy: Policy = ... ; 
    /// 
    /// PolicyBuilder::new()
    ///     .require_any(&[MyScope::Foo, &policy, MyScope::Bar])
    /// ```
    pub fn require_any(self, scopes: impl IntoIterator<Item = impl IntoPolicy<S>>) -> Self {
        let all = Policy::OneOf(scopes.into_iter().map(IntoPolicy::into_policy).collect());
        
        let policy = match self.0 {
            Some(p) => p & all,
            None => all,
        };
        
        Self(Some(policy))
    }

    /// Require another policy
    /// 
    /// This is equivalent to `policy & other`, or to calling
    /// [PolicyBuilder::require_all] with a single item
    pub fn and(self, other: impl IntoPolicy<S>) -> Self {
        self.require_all([other])
    }

    /// Add an alternative policy
    /// 
    /// This is equivalent to `policy | other`
    pub fn or(self, other: impl IntoPolicy<S>) -> Self {
        let policy = match self.0 {
            Some(p) => p | other.into_policy(),
            None => other.into_policy(),
        };
        Self(Some(policy))
    }
}

impl<S: Scope> Default for PolicyBuilder<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Scope> IntoPolicy<S> for PolicyBuilder<S> {
    fn into_policy(self) -> Policy<S> {
        self.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::{IntoPolicy, Policy, PolicyBuilder};

    #[test]
    fn test_default_policy() {
        let policy = PolicyBuilder::<String>::new().build();

        assert_eq!(Policy::<String>::DenyAll, policy);
    }

    #[test]
    fn test_default_builder() {
        assert_eq!(PolicyBuilder::<String>::default(), PolicyBuilder::<String>::new())
    }

    #[test]
    fn test_syntactic_sugar() {

        // Constructors

        let scopes = vec!["foo".to_string(), "bar".to_string()];

        assert_eq!(PolicyBuilder::all_of(&scopes), PolicyBuilder::new().require_all(&scopes));
        assert_eq!(PolicyBuilder::one_of(&scopes), PolicyBuilder::new().require_any(&scopes));

        let policy = "foo".to_string().into_policy();

        assert_eq!(PolicyBuilder::not(policy.clone()), PolicyBuilder::from_policy(!policy));

        let builder = PolicyBuilder::from_policy("foo".to_string());

        assert_eq!(
            builder.clone().and("bar".to_string()),
            builder.clone().require_all(["bar".to_string()])
        );

        assert_eq!(
            builder.clone().or("bar".to_string()),
            PolicyBuilder::from_policy("foo".to_string().into_policy() | "bar".to_string())
        );

        let scope = "foo".to_string();

        assert_eq!(PolicyBuilder::new().require(scope.clone()), PolicyBuilder::from_policy(&scope));
    }

    #[test]
    fn test_complex_policy() {

        let expected = Policy::OneOf(vec![
            Policy::Scope("admin".to_string()),
            Policy::AllOf(vec![
                Policy::Scope("user".to_string()), 
                !Policy::Scope("muted".to_string()),
            ]),
            Policy::AllOf(vec![
                Policy::OneOf(vec![
                    Policy::Scope("bar".to_string()),
                    Policy::Scope("baz".to_string())
                ]),
                Policy::Scope("foo".to_string()),
            ]),
        ]);

        let policy = PolicyBuilder::new()
            .require("admin".to_string())
            .or(
                PolicyBuilder::not("muted".to_string()).and("user".to_string())
            )
            .or(
                PolicyBuilder::new()
                    .require("foo".to_string())
                    .and(
                        PolicyBuilder::one_of([
                            "bar".to_string(),
                            "baz".to_string()
                        ])
                    )
            )
            .build();

        assert_eq!(expected, policy);
    }
}
