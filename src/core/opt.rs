use clap::Clap;

/// Generates a hypergraph according to the
/// random preferential attachment hypergraph model with vertex deactivation
#[derive(Clap, Debug)]
#[clap(name = "hypergraphs")]
pub struct Opt {
    /// Probability of the vertex arrival event
    #[clap(default_value = "0.30")]
    pub pv: f64,
    /// Probability of the edge arrival event
    #[clap(default_value = "0.49")]
    pub pe: f64,
    /// Probability of the vertex deactivation event
    #[clap(default_value = "0.21")]
    pub pd: f64,

    /// Size of hyperedges
    #[clap(default_value = "3")]
    pub m: usize,

    /// Number of iterations to perform
    #[clap(default_value = "1000")]
    pub t: u64,

    /// Number of hypergraphs to generate
    #[clap(long, default_value = "5")]
    pub runs: u32,

    /// Whether hypergraphs should be generated in parallel
    #[clap(long)]
    pub par: bool,

    /// Number of retries to perform until the model finishes with success
    #[clap(long, default_value = "100")]
    pub retries: u32,

    /// Template path to a JSON file to save the generated hypergraph to
    #[clap(long, default_value = "data/hypergraph")]
    pub save: String,
}