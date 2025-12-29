use crate::{policy::{IntoPolicy, Policy}, scope::Scope};


impl<S: Scope> Policy<S> {
    pub fn builder() -> PolicyBuilder<S> {
        PolicyBuilder::new()
    }
}

pub struct PolicyBuilder<S: Scope>(Option<Policy<S>>);

impl<S: Scope> PolicyBuilder<S> {

    /// Create a new PolicyBuilder.
    /// 
    /// The default policy produced by an unmodified builder is a policy that rejects everything.
    pub fn new() -> Self {
        Self(None)
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

impl<S: Scope> IntoPolicy<S> for PolicyBuilder<S> {
    fn into_policy(self) -> Policy<S> {
        self.build()
    }
}
