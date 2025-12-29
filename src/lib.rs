pub mod scope;
pub mod hierarchy;
pub mod policy;

pub mod derive {
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
