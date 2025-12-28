extern crate proc_macro2;

use std::collections::HashMap;

use darling::{FromDeriveInput, FromVariant, ast};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use crate::scope::Scope;

#[cfg(feature = "hierarchy")]
mod hierarchy;

mod scope;

// Options for the enum to be derived
#[derive(FromDeriveInput)]
#[darling(attributes(scope), supports(enum_unit))]
struct ScopeOpts {
    ident: syn::Ident,

    #[darling(default = || ".".to_string())]
    separator: String,

    #[darling(default)]
    prefix: String,

    #[cfg(feature = "hierarchy")]
    #[darling(default = || true)]
    hierarchy: bool,

    data: ast::Data<ScopeVariantOpts, ()>,
}

// Options for an enum variant of the scope enum
#[derive(Debug, FromVariant)]
#[darling(attributes(scope))]
struct ScopeVariantOpts {
    ident: syn::Ident,

    rename: Option<String>,
}

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
            type Err = ();

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#scopes_full_names => Ok(#enum_ident::#scopes_ident),)*
                    _ => Err(()),
                }
            }
        }
    };

    let scope_impl = quote! {
        impl ::scopes_rs::scope::Scope for #enum_ident {}
    };

    let mut scope_impl = quote! {
        #fromstr_impl
        #scope_impl
    };

    // Add Hierarchy implementation if the feature is enabled
    #[cfg(feature = "hierarchy")]
    if opts.hierarchy {
        use quote::TokenStreamExt;

        use crate::hierarchy::implement_hierarchized;

        scope_impl.append_all(implement_hierarchized(enum_ident, &scopes));
    }

    scope_impl.into()
}
