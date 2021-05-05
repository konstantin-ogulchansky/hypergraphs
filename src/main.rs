mod fenwick;
mod hypergraph;
mod cli;

use crate::hypergraph::Hypergraph;
use crate::cli::Opt;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use serde_json::{json, to_string};
use clap::Clap;

fn main() {
    let opt = Opt::parse();

    // Generate a hypergraph.
    let instant    = Instant::now();
    let mut random = Pcg64Mcg::from_entropy();
    let mut result = Hypergraph::generate(opt.pv, opt.pe, opt.pd, |_| opt.m, opt.t, &mut random);

    for _ in 1..=opt.retries {
        if result.is_ok() {
            break;
        }

        result = Hypergraph::generate(opt.pv, opt.pe, opt.pd, |_| opt.m, opt.t, &mut random);
    }

    let generated = result.expect(
        format!("Couldn't generate a hypergraph after {} retries", opt.retries).as_str()
    );

    println!("Elapsed: {}ms", instant.elapsed().as_millis());

    // Save the generated hypergraph into a file.
    let json = json!({
        "parameters": {
            "pv": opt.pv,
            "pe": opt.pe,
            "pd": opt.pd,
            "m":  opt.m,
            "t":  opt.t,
        },
        "nodes":  generated.nodes,
        "edges":  generated.edges,
        "degree": generated.degree,
        "theta":  generated.theta,
    });
    let data = to_string(&json).expect("Couldn't convert to JSON");

    File::create(opt.save).expect("Couldn't create a file")
        .write_all(data.as_bytes()).expect("Couldn't write to the file");
}