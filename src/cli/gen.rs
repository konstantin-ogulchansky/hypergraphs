use crate::core::{
    hypergraph::Hypergraph,
    model::Model,
    simulation::Simulation
};

use std::{fs::File, io::Write, time::Instant};

use clap::Clap;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use rayon::prelude::*;
use serde_json;

/// Generates a hypergraph according to the specified model
#[derive(Clap, Debug)]
pub struct Gen {
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

impl Gen {
    /// Executes the `gen` subcommand.
    pub fn execute(self: &Self) {
        let instant = Instant::now();

        if self.par {
            (0..self.runs)
                .into_par_iter()
                .for_each(|i| self.generate(i).unwrap());
        }
        else {
            (0..self.runs)
                .for_each(|i| self.generate(i).unwrap());
        }

        println!("Total: {:?} elapsed", instant.elapsed());
    }

    /// Generates a hypergraph.
    fn generate(self: &Self, i: u32) -> Result<(), &'static str> {
        let model = Model::new(self.pv, self.pe, self.pd, self.m)?;
        let instant = Instant::now();
        let simulation = model.generate(self.t, self.retries)?;

        println!("[{}]: {:?} elapsed", i, instant.elapsed());

        let data = serde_json::to_string(&simulation).unwrap();
        let path = format!("{}-{}.json", self.save, i);
        let mut file = File::create(&path).unwrap();

        file.write_all(data.as_bytes()).unwrap();

        Ok(())
    }
}