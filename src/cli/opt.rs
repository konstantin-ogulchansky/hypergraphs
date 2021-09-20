use crate::cli::{gen::Gen, plot::Plot};

use clap::Clap;

/// Generates a hypergraph according to the
/// random preferential attachment hypergraph model with vertex deactivation
#[derive(Clap, Debug)]
#[clap(name = "hypergraphs")]
pub struct Opt {
    #[clap(subcommand)]
    pub command: Subcommand
}

#[derive(Clap, Debug)]
pub enum Subcommand {
    Gen(Gen),   // Generates a hypergraph.
    Plot(Plot), // Plots a hypergraph.
}