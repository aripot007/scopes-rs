
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
pub trait Hierarchized {

    // TODO: Add example
    /// Check if a scope includes another one.
    /// 
    /// A parent scope includes its children and itself.
    fn includes(&self, other: &Self) -> bool;


    /// Check if this scope is included in another one.
    /// 
    /// This is equivalent to using [Hierarchized::includes] on the other scope.
    /// 
    /// ```
    /// enum MyScope {
    ///     // ...
    /// }
    /// 
    /// let scope_a = MyScope::A;
    /// let scope_b = MyScope::B;
    /// 
    /// assert_eq!(scope_a.included_in(&scope_b), scope_b.includes(&scope_a))
    /// ```
    #[inline]
    fn included_in(&self, other: &Self) -> bool {
        other.includes(self)
    }
}
