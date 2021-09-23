use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Hypergraph {
    /// The number of vertices in the hypergraph.
    pub vertices: u32,

    /// A vector of hyperedges in the hypergraph.
    /// Each hyperedge is represented as a vector of vertices;
    /// hence, a single hyperedge may contain the same vertex several times.
    pub edges: Vec<Vec<u32>>,

    /// A vector of degrees of vertices (both active and inactive).
    pub degree: Vec<u32>,
}

impl Hypergraph {
    /// Constructs the initial hypergraph that contains a single vertex
    /// with a single hyperedge of size 1.
    pub fn initial() -> Box<Self> {
        Box::new(Self {
            vertices: 1,
            edges:  vec![vec![0]],
            degree: vec![1],
        })
    }

    /// Adds a new vertex.
    ///
    /// # Returns
    /// The index of the added vertex.
    pub fn add_vertex(self: &mut Self) -> u32 {
        let v = self.vertices;

        self.vertices += 1;
        self.degree.push(0);

        v
    }

    /// Adds a new hyperedge.
    ///
    /// # Arguments
    /// * `e` - the hyperedge to add.
    pub fn add_edge(self: &mut Self, e: Vec<u32>) {
        for &v in e.iter() {
            self.degree[v as usize] += 1;
        }

        self.edges.push(e);
    }

    /// Computes the degree distribution of the hypergraph.
    pub fn degree_distribution(self: &Self) -> HashMap<u32, f32> {
        let mut map = HashMap::new();

        for &deg in &self.degree {
            *map.entry(deg).or_insert(0.0) += 1.0;
        }
        for value in map.values_mut() {
            *value /= self.vertices as f32;
        }

        map
    }
}