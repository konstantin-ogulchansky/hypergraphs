use crate::core::{
    hypergraph::Hypergraph,
    fenwick::Fenwick,
    simulation::Simulation
};

use std::error::Error;

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use serde::{Serialize, Deserialize};

/// The random preferential attachment hypergraph model with vertex deactivation,
/// described by a 5-tuple `H(H_0, p_v, p_e, p_d, Y)`.
///
/// The model works as follows. Given the initial hypergraph `H_0`,
/// construct a hypergraph `H_t` from `H_{t-1}`:
/// - wp `p_v`, add a vertex and a preferentially selected hyperedge of size `Y_t`,
/// - wp `p_e`, add a preferentially selected hyperedge of size `Y_t`,
/// - wp `p_d`, deactivate a preferentially selected vertex.
///
/// Preferential selection means that the probability to select a vertex is proportional to its
/// degree. That is, `P(v is chosen) = deg v / sum [deg u, u in V]`. However, in this model,
/// we only preferentially select among active vertices.
///
/// The described model generates hypergraphs whose degree distribution follows a power-law
/// distribution with an exponential cutoff.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Probability of the vertex arrival event.
    pub pv: f64,

    /// Probability of the edge arrival event.
    pub pe: f64,

    /// Probability of the vertex deactivation event.
    pub pd: f64,

    /// Sizes of hyperedges.
    pub m: usize,
}

impl Model {
    /// Creates a model with the specified parameters.
    /// Ensures that the provided parameters are correct.
    pub fn new(pv: f64, pe: f64, pd: f64, m: usize) -> Result<Model, Box<dyn Error>> {
        if pv < 0.0 || pe < 0.0 || pd < 0.0 {
            return Err("Expected `pv`, `pe` and `pd` to be positive".into());
        }
        if f64::abs(pv + pe + pd - 1.0) >= f64::EPSILON {
            return Err("Expected `pv`, `pe` and `pd` to sum up to 1".into());
        }
        if pv <= pd {
            return Err("Expected `pv > pd` to hold".into());
        }
        if m < 1 {
            return Err("Expected `m` to be a positive integer".into());
        }

        Ok(Model { pv, pd, pe, m })
    }

    /// The size of a hyperedge at step `t`.
    pub fn size(self: &Self, t: u64) -> usize {
        self.m
    }

    /// Generates a hypergraph according to the model.
    ///
    /// # Arguments
    /// * `steps` - the number of steps of a simulation to perform.
    /// * `retries` - the number of times to retry a simulation;
    ///               simulation failures are expected to happen, for example,
    ///               if we randomly deactivate all active vertices.
    ///
    /// # Returns
    /// A `Result` instance that contains either a simulation result or an error message.
    pub fn generate(self: &Self, steps: u64, retries: u32) -> Result<Simulation, Box<dyn Error>> {
        let mut random = Pcg64Mcg::from_entropy();
        let mut result = Simulation::run(self, steps, &mut random);

        // Retry in case if a simulation fails.
        for _ in 0..retries {
            if result.is_err() {
                result = Simulation::run(self, steps, &mut random);
            }
            else {
                break;
            }
        }

        result
    }

    /// Computes the theoretical degree distribution.
    pub fn degree_distribution(self: &Self) -> Box<dyn Fn(f64) -> f64> {
        let pv = self.pv;
        let pe = self.pe;
        let pd = self.pd;
        let m = self.m as f64;

        let g = (pv * (m - 1.) + pe * m) / (pv * (m - 1.) + pe * m + pd);
        let o = self.theta(0., 100000);
        let d = pd / ((pv + pe) * m - pd * o);
        let b = (m * (pv + pe) - pd * o) / (pv * (m - 1.) + pe * m + pd);
        let c = pv / g * gamma(1. + 1./b) / b;

        // The gamma function.
        fn gamma(x: f64) -> f64 {
            panic!("Not implemented.");
        }

        Box::new(move |x| c/pv * g.powf(x)/x.powf(1.0/ b) * (1.0/x + d))
    }

    /// Computes `theta` using the fixed-point iteration method.
    fn theta(self: &Self, seed: f64, n: u32) -> f64 {
        // May require the `rgsl` package and GNU Scientific Library.
        // Unfortunately, it appears that there are no alternative libraries,
        // which implement the hypergeometric function.
        panic!("Not implemented.");
    }
}