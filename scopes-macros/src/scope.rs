use syn::Ident;

use crate::{ScopeOpts, ScopeVariantOpts};

// TODO: Implementation without cloning separator and prefix if feasible
#[derive(PartialEq)]
#[cfg_attr(test,derive(Debug))]
pub struct Scope {

    // Ident of the corresponding enum variant
    pub ident: Ident,

    // List of labels comprising the scope, used to determine hierarchy
    #[cfg(feature = "hierarchy")]
    pub labels: Vec<String>,

    // We dont need to split labels without hierarchy
    #[cfg(not(feature = "hierarchy"))]
    pub scope_name: String,

    // Separator for the labels
    // Only useful if hierarchy is enabled to reconstruct the full name
    #[cfg(feature = "hierarchy")]
    separator: String,

    // Name prefix
    prefix: String,

    // List of additional scopes to include
    #[cfg(feature = "hierarchy")]
    include: Vec<syn::Ident>,
}

// Extract a list of labels from an enum variant ident.
// This splits the name of the variant at each capitalized letter
fn get_labels_from_ident(ident: &Ident) -> Vec<String> {

    let mut labels = Vec::new();
    let mut current_label = String::new();

    for (i, ch) in ident.to_string().char_indices() {

        if i > 0 && ch.is_uppercase() {
            labels.push(current_label);
            current_label = String::new();
        }

        current_label.push(ch.to_ascii_lowercase());
    }
    labels.push(current_label);

    return labels;
}


impl Scope {

    pub fn from_variant(variant_opts: &ScopeVariantOpts, opts: &ScopeOpts) -> Self {
        
        let labels = match &variant_opts.rename {

            // If hierarchy is not enabled, we don't need to extract the labels from the name
            #[cfg(not(feature = "hierarchy"))]
            Some(name) => {
                return Self {
                    ident: variant_opts.ident.clone(),
                    scope_name: name.clone(),
                    prefix: opts.prefix.clone(),
                };
            },

            // If hierarchy is enabled, extract the labels from the given name
            #[cfg(feature="hierarchy")]
            Some(name) => name.split(&opts.separator).map(String::from).collect(),

            None => get_labels_from_ident(&variant_opts.ident),
        };

        return Self {
            ident: variant_opts.ident.clone(),
            prefix: opts.prefix.clone(),
            
            #[cfg(feature = "hierarchy")]
            separator: opts.separator.clone(),
            
            #[cfg(feature = "hierarchy")]
            labels,

            #[cfg(feature = "hierarchy")]
            include: variant_opts.include.as_ref().map(|i| i.0.clone()).unwrap_or(Vec::new()),
            
            #[cfg(not(feature = "hierarchy"))]
            scope_name: labels.join(&opts.separator),
        }
    }

    pub fn name(&self) -> String {
        #[cfg(not(feature = "hierarchy"))]
        return self.scope_name.clone();

        #[cfg(feature = "hierarchy")]
        return self.labels.join(&self.separator);
    }

    pub fn full_name(&self) -> String {
        #[cfg(not(feature = "hierarchy"))]
        return self.prefix.clone() + &self.scope_name;

        #[cfg(feature = "hierarchy")]
        return self.prefix.clone() + &self.labels.join(&self.separator);
    }

}

#[cfg(test)]
mod tests {

    use std::vec;

    use darling::ast;
    use proc_macro2::Span;

    use crate::{IncludeList, Scope, ScopeOpts, ScopeVariantOpts, scope::get_labels_from_ident};

    // Implement utility functions to create new scopes in tests
    impl Scope {
        #[cfg(feature = "hierarchy")]
        pub fn _test_new(ident: syn::Ident, labels: impl Iterator<Item = impl AsRef<str>>, separator: impl AsRef<str>, prefix: impl AsRef<str>) -> Self {
            Self {
                ident,
                labels: labels.map(|s| String::from(s.as_ref())).collect(),
                separator: separator.as_ref().to_owned(),
                prefix: prefix.as_ref().to_owned(),
                include: Vec::new(),
            }
        }

        #[cfg(not(feature = "hierarchy"))]
        pub fn _test_new(ident: syn::Ident, name: impl AsRef<str>, prefix: impl AsRef<str>) -> Self {
            Self {
                ident,
                prefix: prefix.as_ref().to_owned(),
                scope_name: name.as_ref().to_owned(),
            }
        }

        // Create a scope struct corresponding to the enabled features
        #[allow(unused_variables)]
        pub fn _test_new_full(ident: syn::Ident, name: impl AsRef<str>, labels: impl IntoIterator<Item = impl AsRef<str>>, separator: impl AsRef<str>, prefix: impl AsRef<str>, include: Vec<syn::Ident>) -> Self {
            Self {
                ident,
                prefix: prefix.as_ref().to_owned(),

                #[cfg(not(feature = "hierarchy"))]
                scope_name: name.as_ref().to_owned(),

                #[cfg(feature = "hierarchy")]
                labels: labels.into_iter().map(|s| String::from(s.as_ref())).collect(),
                #[cfg(feature = "hierarchy")]
                separator: separator.as_ref().to_owned(),

                #[cfg(feature = "hierarchy")]
                include,
            }
        }
    }

    // Create an identifier
    macro_rules! ident {
        ($s: ident) => {
            syn::Ident::new(stringify!($s), Span::call_site())
        };
    }

    fn default_opts() -> ScopeOpts {
        ScopeOpts {
            ident: ident!(ScopeEnum),
            vis: syn::Visibility::Inherited,
            separator: ".".to_string(),
            prefix: "".to_string(),
            
            #[cfg(feature = "hierarchy")]
            hierarchy: false,

            scope_name_getter: true,

            data: ast::Data::Enum(Vec::new()),
        }
    }

    #[test]
    fn test_get_labels_simple() {
        assert_eq!(vec!["foo"], get_labels_from_ident(&ident!(Foo)));
        assert_eq!(vec!["foo", "bar"], get_labels_from_ident(&ident!(FooBar)));
        assert_eq!(vec!["foo"], get_labels_from_ident(&ident!(foo)));
        assert_eq!(vec!["foo_bar"], get_labels_from_ident(&ident!(foo_bar)));
    }

    #[test]
    fn test_get_labels_consecutive_uppercase() {
        assert_eq!(vec!["h", "e", "l", "l", "o"], get_labels_from_ident(&ident!(HELLO)));
    }

    #[test]
    fn test_from_variant() {

        let opts = default_opts();
        
        let variant_opts = ScopeVariantOpts {
            ident: ident!(Foo),
            rename: None,
            include: None,
        };
        assert_eq!(
            Scope::from_variant(&variant_opts, &opts),
            Scope::_test_new_full(ident!(Foo), "foo", get_labels_from_ident(&ident!(Foo)).iter(), &opts.separator, &opts.prefix, vec![])
        );


        let variant_opts = ScopeVariantOpts {
            ident: ident!(FooBar),
            rename: None,
            include: None,
        };
        assert_eq!(
            Scope::from_variant(&variant_opts, &opts),
            Scope::_test_new_full(ident!(FooBar), "foo.bar", get_labels_from_ident(&ident!(FooBar)).iter(), &opts.separator, &opts.prefix, vec![])
        );
    }

    #[test]
    fn test_from_variant_rename() {
        let opts = default_opts();
        
        let variant_opts = ScopeVariantOpts {
            ident: ident!(FooBar),
            rename: Some("baz".to_string()),
            include: Some(IncludeList(Vec::new())),
        };
        assert_eq!(
            Scope::from_variant(&variant_opts, &opts),
            Scope::_test_new_full(ident!(FooBar), "baz", vec!["baz"], &opts.separator, &opts.prefix, vec![])
        );

        let variant_opts = ScopeVariantOpts {
            ident: ident!(FooBar),
            rename: Some("baz.bar".to_string()),
            include: Some(IncludeList(Vec::new())),
        };
        assert_eq!(
            Scope::from_variant(&variant_opts, &opts),
            Scope::_test_new_full(ident!(FooBar), "baz.bar", vec!["baz", "bar"], &opts.separator, &opts.prefix, vec![])
        );
    }

    #[test]
    fn test_name() {
        let mut opts = default_opts();
        opts.prefix = "myprefix/".to_string();

        let foo = Scope::from_variant(&ScopeVariantOpts {ident: ident!(Foo), rename: None, include: None,}, &opts);
        let foo_bar_baz = Scope::from_variant(&ScopeVariantOpts {ident: ident!(FooBarBaz), rename: None, include: None,}, &opts);
        let renamed = Scope::from_variant(&ScopeVariantOpts {ident: ident!(Foo), rename: Some("renamed".to_string()), include: None,}, &opts);
        let renamed_baz = Scope::from_variant(&ScopeVariantOpts {ident: ident!(Foo), rename: Some("renamed.baz".to_string()), include: None,}, &opts);

        assert_eq!("foo", foo.name());
        assert_eq!("foo.bar.baz", foo_bar_baz.name());
        assert_eq!("renamed", renamed.name());
        assert_eq!("renamed.baz", renamed_baz.name());

        assert_eq!("myprefix/foo", foo.full_name());
        assert_eq!("myprefix/foo.bar.baz", foo_bar_baz.full_name());
        assert_eq!("myprefix/renamed", renamed.full_name());
        assert_eq!("myprefix/renamed.baz", renamed_baz.full_name());
    }
}
