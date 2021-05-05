mod fenwick;
mod hypergraph;

use crate::hypergraph::Hypergraph;

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use serde_json::{json, to_string};
use clap::clap_app;

fn main() {
    // Parse command line arguments.
    let matches = clap_app!(app =>
        (version: "1.0")
        (author: "K.")
        (about: "Generates a hypergraph according to the hypergraph model with vertex ageing")

        (@arg pv: +required "Probability of the `vertex-arrival` event")
        (@arg pe: +required "Probability of the `edge-arrival` event")
        (@arg pd: +required "Probability of the `vertex-deactivation` event")
        (@arg m:  +required "Distribution of cardinalities of hyperedges")
        (@arg t:  +takes_value default_value("100000") "Number of iterations to perform")
        (@arg save: --save +takes_value default_value("hypergraph.json")
            "Path to a JSON file to save the generated hypergraph to")
        (@arg retry: --retry
            "Whether to retry until the model finishes with success")
    ).get_matches();

    let pv = matches.value_of_t_or_exit::<f64>("pv");
    let pe = matches.value_of_t_or_exit::<f64>("pe");
    let pd = matches.value_of_t_or_exit::<f64>("pd");
    let m  = matches.value_of_t_or_exit::<u64>("m") as usize;
    let t  = matches.value_of_t_or_exit::<u64>("t");
    let save = matches.value_of_os("save").expect("`save` must be a valid path to a file");
    let retry = matches.is_present("retry");

    // Generate a hypergraph.
    let instant    = Instant::now();
    let mut random = Pcg64Mcg::from_entropy();
    let mut result = Hypergraph::generate(pv, pe, pd, |_| m, t, &mut random);

    while retry && result.is_err() {
        result = Hypergraph::generate(pv, pe, pd, |_| m, t, &mut random);
    }

    let generated = result.unwrap();

    println!("Elapsed: {}ms.", instant.elapsed().as_millis());

    // Save the generated hypergraph into a file.
    let json = json!({
        "parameters": {
            "pv": pv,
            "pe": pe,
            "pd": pd,
            "m":  m,
            "t":  t,
        },
        "nodes":  generated.nodes,
        "edges":  generated.edges,
        "degree": generated.degree,
        "theta":  generated.theta,
    });
    let data = to_string(&json).expect("Couldn't convert to JSON.");

    File::create(save).expect("Couldn't create a file.")
        .write_all(data.as_bytes()).expect("Couldn't write to the file.");
}