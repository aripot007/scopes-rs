
// TODO: Better documentation
/// A trait used to hierarchize scopes.
/// 
/// The implementation must respect the following conditions : 
/// 
/// - A scope must always include itself : `Scope::A.includes(&Scope::A) == true`
/// - The relation is transitive, if scope A includes scope B and scope B includes scope C, `Scope::A.includes(&Scope::C)` must be `true`
/// 
/// You only need to implement [Hierarchized::includes], as the default implementation of
/// [Hierarchized::included_in] just calls [Hierarchized::includes].
#[cfg(feature = "hierarchy")]
pub trait Hierarchized {

    /// Check if a scope includes another one.
    /// 
    /// A parent scope includes its children and itself.
    /// 
    /// ```
    /// # use scopes_macros::Scope;
    /// # use scopes_rs::hierarchy::Hierarchized;
    /// 
    /// #[derive(PartialEq, Scope)]
    /// enum MyScope {
    ///     Foo,
    ///     FooBar,
    ///     Baz,
    /// }
    /// 
    /// assert!(MyScope::Foo.includes(&MyScope::FooBar));
    /// assert!(!MyScope::Foo.includes(&MyScope::Baz));
    /// ```
    /// 
    fn includes(&self, other: &Self) -> bool;


    /// Check if this scope is included in another one.
    /// 
    /// This is equivalent to using [Hierarchized::includes] on the other scope.
    /// 
    /// ```
    /// # use scopes_macros::Scope;
    /// # use scopes_rs::hierarchy::Hierarchized;
    /// 
    /// #[derive(PartialEq, Scope)]
    /// enum MyScope {
    ///     A,
    ///     B,
    /// }
    /// 
    /// let scope_a = MyScope::A;
    /// let scope_b = MyScope::B;
    /// 
    /// assert_eq!(scope_a.included_in(&scope_b), scope_b.includes(&scope_a));
    /// ```
    #[inline]
    fn included_in(&self, other: &Self) -> bool {
        other.includes(self)
    }
}
