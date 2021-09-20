use clap::Clap;

/// Plots the degree distribution of the specified hypergraph
#[derive(Clap, Debug)]
pub struct Plot {
    /// Path to a file to read the hypergraph from
    pub path: String,

    /// Path to a file to save the plot to
    #[clap(long)]
    pub save: String,
}

impl Plot {
    // Executes the `plot` subcommand.
    pub fn execute(self: &Self) {
        panic!("Not implemented.");
    }
}