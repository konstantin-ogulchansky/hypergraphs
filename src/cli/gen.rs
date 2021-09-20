use crate::core::hypergraph::Hypergraph;

use std::{fs::File, io::Write, time::Instant};

use clap::Clap;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use rayon::prelude::*;
use serde_json::{json, to_string};

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
    // Executes the `gen` subcommand.
    pub fn execute(self: &Self) {
        let instant = Instant::now();

        if self.par {
            (0..self.runs)
                .into_par_iter()
                .for_each(|i| self.generate(i));
        }
        else {
            (0..self.runs)
                .for_each(|i| self.generate(i));
        }

        println!("Total: {:?} elapsed", instant.elapsed());
    }

    // Generates a hypergraph.
    fn generate(self: &Self, i: u32) {
        let instant = Instant::now();
        let mut random = Pcg64Mcg::from_entropy();
        let mut result = Hypergraph::generate(
            self.pv, self.pe, self.pd, |_| self.m, self.t, &mut random
        );

        // Regenerate a hypergraph until success.
        for _ in 0..self.retries {
            if result.is_ok() {
                break;
            }

            result = Hypergraph::generate(
                self.pv, self.pe, self.pd, |_| self.m, self.t, &mut random
            );
        }

        let generated = result.expect(
            format!("[{}]: Failed after {} retries", i, self.retries).as_str()
        );

        println!("[{}]: {:?} elapsed", i, instant.elapsed());

        self.save(i, generated);
    }

    // Saves the generated hypergraph to a file.
    fn save(self: &Self, i: u32, generated: Box<Hypergraph>) {
        let json = json!({
            "parameters": {
                "pv": self.pv,
                "pe": self.pe,
                "pd": self.pd,
                "m":  self.m,
                "t":  self.t,
            },
            "vertices": generated.vertices,
            "edges":    generated.edges,
            "degree":   generated.degree,
            "theta":    generated.theta,
        });
        let data = to_string(&json).expect("Couldn't convert to JSON");
        let path = format!("{}-{}.json", self.save, i);
        let mut file = File::create(&path).expect("Couldn't create a file");

        file.write_all(data.as_bytes()).expect("Couldn't write to the file");
    }
}