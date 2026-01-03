use std::collections::{HashMap, HashSet};

use crate::Scope;


// A graph representing scope inclusions
pub struct InclusionGraph<'a> {
    // Adjacency list for each node
    neighbors: HashMap<syn::Ident, HashSet<syn::Ident>>,
    
    // Nodes may not be resolved if the scope wasn't added to the graph yet
    resolved_nodes: HashMap<syn::Ident, &'a Scope>
}

impl<'a> InclusionGraph<'a> {

    pub fn new() -> Self {
        Self {
            neighbors: HashMap::new(),
            resolved_nodes: HashMap::new(),
        }
    }

    // Create an inclusion graph from a list of scopes
    pub fn from_scopes(scopes: impl Iterator<Item = &'a Scope>) -> Self {
        let mut g = Self::new();
        for scope in scopes {
            g.add(scope);
        }
        g
    }

    // Add a scope to the inclusion graph
    pub fn add(&mut self, scope: &'a Scope) {
        
        // Resolve label inclusions
        let mut included_in: Vec<&syn::Ident> = Vec::new();
        let mut includes: Vec<&syn::Ident> = Vec::new();

        for other_scope in self.resolved_nodes.values() {

            if *other_scope == scope {
                continue;
            }

            // other_scope is included in scope
            if other_scope.labels.starts_with(&scope.labels) {
                includes.push(&other_scope.ident);
            
            // scope is included in other_scope
            } else if scope.labels.starts_with(&other_scope.labels) {
                included_in.push(&other_scope.ident);
            }

        }

        for other in included_in {
            self.add_inclusion(other, &scope.ident);
        }

        for other in includes {
            self.add_inclusion(&scope.ident, other);
        }


        // Add manually included scopes
        for other in &scope.include {
            self.add_inclusion(&scope.ident, &other);
        }

        // Add the scope to the resolved nodes if it wasn't resolved already
        self.resolved_nodes.insert(scope.ident.clone(), scope);
    }

    // Get or add a node corresponding to an identifier
    fn get_or_add_node_mut(&mut self, ident: &syn::Ident) -> &mut HashSet<syn::Ident> {
        
        if !self.neighbors.contains_key(ident) {
            self.neighbors.insert(ident.clone(), HashSet::new());
        }
        
        self.neighbors.get_mut(ident).unwrap()
    }

    // Add an edge to the graph, creating the nodes if necessary.
    fn add_inclusion(&mut self, from: &syn::Ident, to: &syn::Ident) {

        let from_node = self.get_or_add_node_mut(from);
        from_node.insert(to.clone());

    }

    // Get all scopes included in scope
    pub fn get_included_in(&self, scope: &syn::Ident) -> impl Iterator<Item = &syn::Ident> {
        DfsIterator::new(self, scope)
    }

    // Iterate over the nodes
    pub fn nodes(&self) -> impl Iterator<Item = &syn::Ident> {
        self.neighbors.keys()
    }

    pub fn has_neighbors(&self, scope: &syn::Ident) -> bool {
        return self.neighbors.get(scope).map(|n| !n.is_empty()).unwrap_or_default()
    }
}

// Depth-first search iterator
struct DfsIterator<'a> {
    graph: &'a InclusionGraph<'a>,
    visited: HashSet<&'a syn::Ident>,
    to_visit: Vec<&'a syn::Ident>,
}

impl<'a> DfsIterator<'a> {
    fn new(graph: &'a InclusionGraph<'_>, start: &syn::Ident) -> Self {
        
        let to_visit = graph.neighbors.get(start).map(|set| set.iter().collect()).unwrap_or_default();

        Self { 
            graph,
            visited: HashSet::new(),
            to_visit,
        }
    }
}

impl<'a> Iterator for DfsIterator<'a> {
    type Item = &'a syn::Ident;

    fn next(&mut self) -> Option<Self::Item> {
        
        let current = match self.to_visit.pop_if(|i| !self.visited.contains(i)) {
            Some(ident) => ident,
            None => return None,
        };

        self.visited.insert(current);

        // Add neighbors
        let neighbors = self.graph.neighbors.get(current).map(HashSet::iter).unwrap_or_default();
        for neighbor in neighbors {
            if !self.visited.contains(neighbor) {
                self.to_visit.push(neighbor);
            }
        }

        Some(current)
    }
}
