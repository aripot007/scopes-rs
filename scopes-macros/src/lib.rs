#![warn(missing_docs)]
//! Macros for [`scopes-rs`](https://github.com/aripot007/scopes-rs)
//!
//! This crate is re-exported by `scopes-rs`, and its documentation should be
//! browsed from the `scopes-rs` crate.
//!  
extern crate proc_macro2;

use std::{collections::HashMap, fmt::Debug};

use darling::{FromDeriveInput, FromVariant, ast};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

#[cfg(feature = "hierarchy")]
use darling::FromMeta;

use crate::scope::Scope;


#[cfg(feature = "hierarchy")]
mod hierarchy;

mod scope;

// Options for the enum to be derived
#[derive(FromDeriveInput)]
#[darling(attributes(scope), supports(enum_unit))]
struct ScopeOpts {
    ident: syn::Ident,
    vis: syn::Visibility,

    #[darling(default = || ".".to_string())]
    separator: String,

    #[darling(default)]
    prefix: String,

    #[cfg(feature = "hierarchy")]
    #[darling(default = || true)]
    hierarchy: bool,

    // Add a function to get the scope name
    #[darling(default = || false)]
    scope_name_getter: bool,

    data: ast::Data<ScopeVariantOpts, ()>,
}

// Options for an enum variant of the scope enum
#[derive(Debug, FromVariant)]
#[darling(attributes(scope))]
struct ScopeVariantOpts {
    ident: syn::Ident,

    rename: Option<String>,

    #[cfg(feature = "hierarchy")]
    include: Option<IncludeList>,
}

#[cfg(test)]
impl Default for ScopeVariantOpts {
    fn default() -> Self {
        Self { 
            ident: syn::parse_quote!(Foo), 
            rename: Default::default(),
            #[cfg(feature = "hierarchy")]
            include: Default::default()
        }
    }
}

#[cfg(feature = "hierarchy")]
#[derive(Debug)]
struct IncludeList(pub Vec<syn::Ident>);

/// ## Optional `#[scope(...)]` attributes for the enum
/// 
/// - `separator = "..."`: Change the separator between scope labels. Defaults to `"."`
/// - `prefix = "..."`: Add a prefix to every generated scope name. Default is an empty prefix
/// - `hierarchy = bool`: Enable or disable generation of the `Hierarchized` trait. Requires the `hierarchy`
///     feature. Defaults to `true`.
/// - `scope_name_getter = bool`: Implement the `scope_name()` function to get the scope name from a variant (defaults to true)
/// 
/// ## Optional `#[scope(...)]` attributes for enum variants
/// 
/// - `rename = "..."`: Use a specific name instead of inferring it from the variant name
/// - `include = scope | [scope1, ...]`: Include other scopes in the hierarchy. See below for more details.
///     requires the `hierarchy` feature
/// 
/// ### Hierarchy customization
/// 
/// The generated scopes hierarchy can be customized with the `#[scope(include = ...)]` attribute.
/// 
/// The attributes takes a scope variant or a list of scope variants that should be included in the scope :
/// 
/// 
/// ```ignore
/// use scopes_rs::{
///     derive::Scope,
///     hierarchy::Hierarchized,
/// };
/// 
/// #[derive(Clone, Debug, PartialEq, Scope)]
/// enum MyScope {
/// 
///     Foo,
///     #[scope(include = FooBarReadonly)]
///     FooReadonly,
/// 
///     FooBar,
///     FooBarReadonly,
///     
///     #[scope(include = [FooReadonly, BarReadonly])]
///     Readonly,
///     
///     Bar,
///     BarReadonly,
/// }
/// 
/// assert!(MyScope::Readonly.includes(&MyScope::BarReadonly));
/// assert!(MyScope::Readonly.includes(&MyScope::FooReadonly));
/// 
/// // Inclusion is transitive, so Readonly also includes FooBarReadonly 
/// // since FooReadonly includes it
/// assert!(MyScope::Readonly.includes(&MyScope::FooBarReadonly))
/// ```
#[proc_macro_derive(Scope, attributes(scope))]
pub fn derive_into_scope(item: TokenStream) -> TokenStream {
    
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let opts = match ScopeOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(err) => {
            return err.write_errors().into()
        },
    };
    
    derive_into_scope_impl(&opts)
}

fn derive_into_scope_impl(opts: &ScopeOpts) -> TokenStream {

    let enum_ident = &opts.ident;

    // Extract enum variants and their options
    let variants = match &opts.data {
        ast::Data::Enum(items) => items,
        ast::Data::Struct(_) => {
            return syn::Error::new(Span::call_site(), "The Scope derive macro only accepts enums").into_compile_error().into();
        },
    };

    // Parse scope names
    let mut scopes: HashMap<String, Scope> = HashMap::with_capacity(variants.len());
    let mut error: Option<syn::Error> = None;

    for variant in variants {
        
        let scope = Scope::from_variant(variant, &opts);
        let scope_full_name = scope.full_name();

        // Raise error for scopes with conflicting names
        if let Some(other_scope) = scopes.get(&scope_full_name) {
            let mut err = syn::Error::new(
                variant.ident.span(), 
                format!("Conflicting scope name '{}' (conflicting with variant {}::{})", scope.name(), enum_ident, &other_scope.ident)
            );

            err.combine(syn::Error::new(
                other_scope.ident.span(), 
                format!("Conflicting scope name '{}' (conflicting with variant {}::{})", other_scope.name(), enum_ident, variant.ident)
            ));

            if let Some(error) = error.as_mut() {
                error.combine(err);
            } else {
                error = Some(err)
            }

        } else {
            scopes.insert(scope_full_name, scope);
        }

    }

    if let Some(err) = error {
        return err.into_compile_error().into();
    }

    // Implement parsing from a string

    let (scopes_full_names, scopes_ident): (Vec<_>, Vec<_>) = scopes
        .iter()
        .map(|(k, v)| (k, &v.ident))
        .unzip();

    let fromstr_impl = quote! {
        impl ::std::str::FromStr for #enum_ident {
            type Err = ::scopes_rs::error::ScopeParseError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#scopes_full_names => Ok(#enum_ident::#scopes_ident),)*
                    _ => Err(::scopes_rs::error::ScopeParseError(s.to_string())),
                }
            }
        }
    };

    // Implement scope_name() function
    let scope_name_impl;
    if opts.scope_name_getter {

        let vis = &opts.vis;

        scope_name_impl = quote! {
            impl #enum_ident {
                #vis const fn scope_name(&self) -> &'static str {
                    match self {
                        #(#enum_ident::#scopes_ident => #scopes_full_names,)*
                    }
                }
            }
        }
    } else {
        scope_name_impl = quote! {}
    }

    let scope_impl = quote! {
        impl ::scopes_rs::scope::Scope for #enum_ident {}
    };

    let scope_impl = quote! {
        #fromstr_impl
        #scope_name_impl
        #scope_impl
    };

    #[cfg(feature = "hierarchy")]
    let mut scope_impl = scope_impl;

    // Add Hierarchy implementation if the feature is enabled
    #[cfg(feature = "hierarchy")]
    if opts.hierarchy {
        use quote::TokenStreamExt;

        use crate::hierarchy::implement_hierarchized;

        scope_impl.append_all(implement_hierarchized(enum_ident, &scopes));
    }

    scope_impl.into()
}

#[cfg(feature = "hierarchy")]
impl FromMeta for IncludeList {

    fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {

        match *expr {
            syn::Expr::Lit(ref lit) => Self::from_value(&lit.lit),
            syn::Expr::Group(ref group) => {
                // syn may generate this invisible group delimiter when the input to the darling
                // proc macro (specifically, the attributes) are generated by a
                // macro_rules! (e.g. propagating a macro_rules!'s expr)
                // Since we want to basically ignore these invisible group delimiters,
                // we just propagate the call to the inner expression.
                Self::from_expr(&group.expr)
            },

            // Single scope
            syn::Expr::Path(_) => Ok(IncludeList(vec![parse_included_scope(expr)?])),

            // List of included scopes
            syn::Expr::Array(syn::ExprArray { ref elems, ..})
            | syn::Expr::Tuple(syn::ExprTuple { ref elems, .. }) => {
                
                // Parse each expression in the list
                let parsed_elems: Vec<syn::Ident> = elems.iter()
                    .map(parse_included_scope)

                    // Fold into a vec or combine errors
                    .fold(Ok(Vec::new()), |acc, r| match (acc, r) {
                        (Ok(mut elts), Ok(e)) => {
                            elts.push(e);
                            Ok(elts)
                        },
                        (Ok(_), Err(e)) => Err(vec![e]),
                        (Err(acc), Ok(_)) => Err(acc),
                        (Err(mut acc), Err(e)) => {
                            acc.push(e);
                            Err(acc)
                        },
                    })
                    .map_err(darling::Error::multiple)?;

                Ok(IncludeList(parsed_elems))
            },

            _ => Err(darling::Error::unexpected_expr_type(expr)),
        }
        .map_err(|e| e.with_span(expr))
    }
}

#[cfg(feature = "hierarchy")]
fn parse_included_scope(expr: &syn::Expr) -> Result<syn::Ident, darling::Error> {
    use syn::spanned::Spanned;

    
    let syn::ExprPath { path, .. } = match expr {
        syn::Expr::Path(p) => p,
        _ => return Err(darling::Error::unexpected_expr_type(expr)),
    };

    let variant_name = match path.segments.len() {
        1 => &path.segments[0],
        2 => &path.segments[1],
        _ => return Err(darling::Error::custom("Expected an enum variant like `MyEnum::Variant` or `Variant`").with_span(&expr.span()))
    };

    Ok(variant_name.ident.clone())
}
