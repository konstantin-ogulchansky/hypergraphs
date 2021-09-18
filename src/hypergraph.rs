use crate::fenwick::Fenwick;

use rand::Rng;

pub struct Hypergraph {
    /// The number of nodes in the hypergraph.
    pub nodes:  u32,
    /// A vector of hyperedges in the hypergraph.
    /// Each hyperedge is represented as a vector of vertices;
    /// hence, a single hyperedge may contain the same vertex several times.
    pub edges:  Vec<Vec<u32>>,
    /// A vector of degrees of vertices (both active and inactive).
    pub degree: Vec<u32>,
    /// A vector of thetas, that is, the expected degrees of deactivated vertices.
    /// The size of this vector equals the number of steps `t`.
    pub theta:  Vec<f64>,

    fenwick: Fenwick,           // A Fenwick tree of degrees of active (!) vertices.
    active_degree_total:  u32,  // Sum of active degrees.
    active_degree_square: u32,  // Sum of squares of active degrees.
}

impl Hypergraph {
    /// Constructs the initial hypergraph that contains a single vertex
    /// with a single hyperedge of size 1.
    ///
    /// # Arguments
    /// * `t` - the number of steps in the simulation.
    pub fn initial(t: usize) -> Self {
        let mut fenwick = Fenwick::of_size(t);

        fenwick.add(0, 1);

        Self {
            nodes:  1,
            edges:  vec![vec![0]],
            degree: vec![1],
            theta:  vec![1.0],

            fenwick,
            active_degree_total:  1,
            active_degree_square: 1,
        }
    }

    /// Generates a hypergraph according to the hypergraph model with vertex ageing
    /// `H(H_0, p_v, p_e, p_d, Y)` after `t` steps.
    ///
    /// Given the initial hypergraph `H_0`, construct a hypergraph `H_t` from `H_{t-1}` as follows:
    /// - wp `pv`, add a vertex and a preferentially selected hyperedge of size `Y_t`,
    /// - wp `pe`, add a preferentially selected hyperedge of size `Y_t`,
    /// - wp `pd`, deactivate a preferentially selected vertex.
    ///
    /// Preferential selection means that the probability to select a vertex is proportional to its
    /// degree. That is, `P(v is chosen) = deg v / sum [deg u, u in V]`. However, in this model,
    /// we only preferentially select among active vertices.
    ///
    /// The described model generates hypergraphs whose degree distribution follows a power-law
    /// distribution with an exponential cutoff.
    ///
    /// # Arguments
    /// * `pv` - probability of the _vertex-arrival_ event;
    /// * `pe` - probability of the _edge-arrival_ event;
    /// * `pd` - probability of the _vertex-deactivation_ event;
    /// * `y`  - distribution of sizes of hyperedges;
    /// * `t`  - the number of steps to perform.
    ///
    /// # Returns
    /// A `Result` instance that contains either a boxed generated hypergraph or an error message.
    pub fn generate<F, R>(
        pv: f64, pe: f64, pd: f64, y: F, t: u64, random: &mut R
    ) -> Result<Box<Self>, &'static str>
        where F: Fn(u64) -> usize, R: Rng + ?Sized
    {
        if pv < 0.0 || pe < 0.0 || pd < 0.0 {
            return Err("Expected `pv`, `pe` and `pd` to be positive");
        }
        if f64::abs(pv + pe + pd - 1.0) >= f64::EPSILON {
            return Err("Expected `pv`, `pe` and `pd` to sum up to 1");
        }
        if pv <= pd {
            return Err("Expected `pv > pd` to hold");
        }

        let mut hypergraph = Box::new(Hypergraph::initial(t as usize));

        for i in 1..=t {
            if hypergraph.active_degree_total == 0 {
                return Err("All vertices have been deactivated");
            }

            let p = random.gen::<f64>();

            // Vertex arrival.
            if p <= pv {
                let v = hypergraph.add_vertex();
                let m = y(i);
                let mut e = hypergraph.fenwick.sample_many(m - 1, random);

                e.push(v);

                hypergraph.add_edge(e);
            }
            // Edge arrival.
            else if p <= pv + pe {
                let m = y(i);
                let e = hypergraph.fenwick.sample_many(m, random);

                hypergraph.add_edge(e);
            }
            // Vertex deactivation.
            else {
                let v = hypergraph.fenwick.sample_one(random);

                hypergraph.deactivate_vertex(v);
            }

            hypergraph.theta.push(
                hypergraph.active_degree_square as f64 /
                hypergraph.active_degree_total  as f64
            );
        }

        Ok(hypergraph)
    }

    /// Adds a new vertex.
    ///
    /// # Returns
    /// The index of the added vertex.
    fn add_vertex(self: &mut Self) -> u32 {
        let v = self.nodes;

        self.nodes += 1;
        self.degree.push(0);

        v
    }

    /// Adds a new hyperedge.
    ///
    /// # Arguments
    /// * `e` - the hyperedge to add.
    fn add_edge(self: &mut Self, e: Vec<u32>) {
        for &v in e.iter() {
            let d = self.degree[v as usize];

            self.degree[v as usize] += 1;

            self.active_degree_total  += 1;
            self.active_degree_square += 2*d + 1;

            self.fenwick.add(v as usize, 1);
        }

        self.edges.push(e);
    }

    /// Deactivates the vertex.
    ///
    /// # Arguments
    /// * `v` - the index of the vertex to deactivate.
    fn deactivate_vertex(self: &mut Self, v: u32) {
        let d = self.degree[v as usize];

        self.active_degree_total  -= d;
        self.active_degree_square -= d * d;

        self.fenwick.set(v as usize, 0);
    }
}
