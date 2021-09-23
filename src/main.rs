mod cli;
mod core;

use crate::cli::{
    opt::{Opt, Subcommand},
    gen::Gen,
    plot::Plot,
};

use clap::Clap;

fn main() {
    let opt = Opt::parse();

    match opt.command {
        Subcommand::Gen(x)  => x.execute(),
        Subcommand::Plot(x) => x.execute(),
    }.unwrap();
}