use crate::core::{
    hypergraph::Hypergraph,
    simulation::Simulation
};

use std::fs;
use std::cmp::Ordering;
use std::ops::Range;

use clap::Clap;
use itertools;
use plotters::prelude::*;
use serde_json;

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
    /// Executes the `plot` subcommand.
    pub fn execute(self: &Self) {
        let file = fs::read_to_string(self.path.as_str()).unwrap();
        let simulation: Simulation = serde_json::from_str(file.as_str()).unwrap();

        self.plot(&simulation);
    }

    /// Plots the degree distribution of the generated hypergraph.
    fn plot(self: &Self, simulation: &Simulation) -> Result<(), Box<dyn std::error::Error>> {
        // Compute the empirical degree distribution to display.
        let distribution = simulation.hypergraph.degree_distribution();

        let x = *distribution.keys().min().unwrap() as f32..
                *distribution.keys().max().unwrap() as f32;
        let y = *distribution.values().min_by(cmp).unwrap()..
                *distribution.values().max_by(cmp).unwrap();

        fn cmp(a: &&f32, b: &&f32) -> Ordering {
            a.partial_cmp(b).unwrap_or(Ordering::Equal)
        }

        // Construct the plot.
        let root = BitMapBackend::new(self.save.as_str(), (640, 480))
            .into_drawing_area();

        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .right_y_label_area_size(40)
            .margin(5)
            .caption("Degree Distribution", ("sans-serif", 18.0).into_font())
            .build_cartesian_2d((1.0..x.end).log_scale(), (y.start..1.0).log_scale())?
            .set_secondary_coord(0f32..10f32, -1.0f32..1.0f32);

        // Configure axes.
        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_desc("Degree `k`")
            .y_label_formatter(&|x| format!("{:e}", x))
            .draw()?;

        // Scatter the empirical degree distribution.
        chart
            .draw_series(
                simulation.hypergraph.degree_distribution()
                    .iter()
                    .map(|(x, y)| Circle::new((*x as f32, *y as f32), 3, BLACK))
            )?
            .label("Empirical distribution")
            .legend(|(x, y)| Circle::new((x, y), 3, BLACK));

        // Configure the legend.
        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        // Show the result.
        root.present()?;

        Ok(())
    }
}