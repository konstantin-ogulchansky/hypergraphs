use crate::fenwick::Fenwick;

use rand::Rng;

/// An event that occurs during each iteration of the model, namely
/// * _vertex-arrival_, which adds a new vertex and a hyperedge to the existing structure;
/// * _edge-arrival_, which adds a new hyperedge;
/// * _vertex-deactivation_, which deactivates an active vertex.
#[derive(Copy, Clone, Debug)]
pub enum Event {
    VertexArrival,
    EdgeArrival,
    VertexDeactivation
}

pub struct Hypergraph {
    pub nodes:  u32,
    pub edges:  Vec<Vec<u32>>,
    pub degree: Vec<u32>,
    pub theta:  Vec<f64>,
}

impl Hypergraph {
    /// Constructs the initial hypergraph that contains a single vertex
    /// with a single hyperedge of size 1.
    pub fn initial() -> Self {
        Self {
            nodes:  1,
            edges:  vec![vec![0]],
            degree: vec![1],
            theta:  vec![],
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
    /// degree. That is, `P(v is chosen) = deg v / sum [deg u, u in V]`.
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

        let mut hypergraph = Box::new(Hypergraph::initial());
        let mut fenwick = Fenwick::of_size(t as usize);

        fenwick.add(0, 1);

        let mut active_degree_total  = 1;
        let mut active_degree_square = 1;

        for i in 1..t {
            if active_degree_total == 0 {
                return Err("All vertices have been deactivated");
            }

            hypergraph.theta.push(
                active_degree_square as f64 /
                active_degree_total  as f64
            );

            let p = random.gen::<f64>();
            let event =
                     if p < pv      { Event::VertexArrival }
                else if p < pv + pe { Event::EdgeArrival }
                else                { Event::VertexDeactivation };

            match event {
                Event::VertexArrival => {
                    let n = hypergraph.nodes;
                    let m = y(i);
                    let mut e = fenwick.sample_many(m - 1, random);

                    e.push(n);

                    hypergraph.nodes += 1;
                    hypergraph.degree.push(0);

                    for &v in e.iter() {
                        let d = hypergraph.degree[v as usize];

                        hypergraph.degree[v as usize] += 1;

                        active_degree_total  += 1;
                        active_degree_square += 2*d + 1;

                        fenwick.add(v as usize, 1);
                    }

                    hypergraph.edges.push(e);
                }

                Event::EdgeArrival => {
                    let m = y(i);
                    let e = fenwick.sample_many(m, random);

                    for &v in e.iter() {
                        let d = hypergraph.degree[v as usize];

                        hypergraph.degree[v as usize] += 1;

                        active_degree_total  += 1;
                        active_degree_square += 2*d + 1;

                        fenwick.add(v as usize, 1);
                    }

                    hypergraph.edges.push(e);
                }

                Event::VertexDeactivation => {
                    let v = fenwick.sample_one(random) as u32;
                    let d = hypergraph.degree[v as usize];

                    active_degree_total  -= d;
                    active_degree_square -= d * d;

                    fenwick.set(v as usize, 0);
                }
            }
        }

        Ok(hypergraph)
    }
}
