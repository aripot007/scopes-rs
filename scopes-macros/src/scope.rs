use syn::Ident;

use crate::{ScopeOpts, ScopeVariantOpts};

// TODO: Implementation without cloning separator and prefix if feasible
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

        current_label.push(ch);
    }

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
