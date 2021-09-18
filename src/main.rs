mod core;

use crate::core::{
    hypergraph::Hypergraph,
    opt::Opt
};

use std::{fs::File, io::Write, time::Instant};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use clap::Clap;
use rayon::prelude::*;
use serde_json::{json, to_string};

// Generates a hypergraph with the specified parameters of the model.
fn generate(i: u32, opt: &Opt) -> Box<Hypergraph> {
    let instant = Instant::now();
    let mut random = Pcg64Mcg::from_entropy();
    let mut result = Hypergraph::generate(opt.pv, opt.pe, opt.pd, |_| opt.m, opt.t, &mut random);

    // Regenerate a hypergraph until success.
    for _ in 1..=opt.retries {
        if result.is_ok() {
            break;
        }

        result = Hypergraph::generate(opt.pv, opt.pe, opt.pd, |_| opt.m, opt.t, &mut random);
    }

    let generated = result.expect(
        format!("[{}]: Couldn't generate a hypergraph after {} retries", i, opt.retries).as_str()
    );

    println!("[{}]: {:?} elapsed", i, instant.elapsed());

    generated
}

// Saves the generated hypergraph to a file.
fn save(i: u32, generated: Box<Hypergraph>, opt: &Opt) {
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
    let path = format!("{}-{}.json", opt.save, i);

    File::create(&path).expect("Couldn't create a file")
        .write_all(data.as_bytes()).expect("Couldn't write to the file");
}

fn main() {
    let opt = Opt::parse();
    let instant = Instant::now();

    if opt.par {
        (0..opt.runs)
            .into_par_iter()
            .for_each(|i| save(i, generate(i, &opt), &opt));
    }
    else {
        (0..opt.runs)
            .for_each(|i| save(i, generate(i, &opt), &opt));
    }

    println!("Total: {:?} elapsed", instant.elapsed());
}