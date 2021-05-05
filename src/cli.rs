use clap::Clap;
use std::ffi::OsString;

/// Generates a hypergraph according to the hypergraph model with vertex ageing
#[derive(Clap, Debug)]
#[clap(name = "hypergraphs")]
pub struct Opt {
    /// Probability of the `vertex-arrival` event
    pub pv: f64,
    /// Probability of the `edge-arrival` event
    pub pe: f64,
    /// Probability of the `vertex-deactivation` event
    pub pd: f64,

    /// Distribution of cardinalities of hyperedges
    pub m: usize,

    /// Number of iterations to perform
    #[clap(default_value = "100000")]
    pub t: u64,

    /// Path to a JSON file to save the generated hypergraph to
    #[clap(long, default_value = "hypergraph.json")]
    pub save: OsString,

    /// Number of retries to perform until the model finishes with success
    #[clap(long, default_value = "5")]
    pub retries: u32,
}