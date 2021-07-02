use clap::Clap;

/// Generates a hypergraph according to the hypergraph model with vertex ageing
#[derive(Clap, Debug)]
#[clap(name = "hypergraphs")]
pub struct Opt {
    /// Probability of the `vertex-arrival` event
    #[clap(default_value = "0.30")]
    pub pv: f64,
    /// Probability of the `edge-arrival` event
    #[clap(default_value = "0.49")]
    pub pe: f64,
    /// Probability of the `vertex-deactivation` event
    #[clap(default_value = "0.21")]
    pub pd: f64,

    /// Distribution of cardinalities of hyperedges
    #[clap(default_value = "3")]
    pub m: usize,

    /// Number of iterations to perform
    #[clap(default_value = "1000")]
    pub t: u64,

    /// Template path to a JSON file to save the generated hypergraph to
    #[clap(long, default_value = "data/hypergraph")]
    pub save: String,

    /// Number of hypergraphs to generate.
    #[clap(long, default_value = "5")]
    pub runs: u32,

    /// Number of retries to perform until the model finishes with success
    #[clap(long, default_value = "1000")]
    pub retries: u32,
}