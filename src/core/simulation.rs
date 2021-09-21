use crate::core::{
    hypergraph::Hypergraph,
    fenwick::Fenwick,
    model::Model
};

use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    /// The model used in the simulation.
    pub model: Model,

    /// The hypergraph generated according to the model.
    pub hypergraph: Box<Hypergraph>,

    /// The number of steps performed.
    pub steps: u64,

    /// A vector of thetas, that is, the expected degrees of deactivated vertices.
    /// The size of this vector equals the number of steps.
    pub theta: Vec<f64>,
}

impl Simulation {
    /// Runs a simulation and generates a hypergraph according to the specified model.
    ///
    /// # Arguments
    /// * `model` - the model according to which a hypergraph should be generated;
    /// * `steps` - the number of steps of the simulation to perform;
    /// * `random` - a random number generator.
    ///
    /// # Returns
    /// A `Simulation` object, which describes the result of the simulation.
    pub fn run<R>(
        model: &Model, steps: u64, random: &mut R
    ) -> Result<Simulation, &'static str>
        where R: Rng + ?Sized
    {
        let mut hypergraph = Hypergraph::initial();
        let mut fenwick = Fenwick::of_size(steps as usize);

        let mut active_squares: u32 = 0;
        let mut active_degrees: u32 = 0;

        // Initialize the Fenwick tree with degrees of active vertices.
        for v in 0..hypergraph.vertices {
            let deg = hypergraph.degree[v as usize];

            fenwick.set(v as usize, deg as i32);

            active_squares += deg * deg;
            active_degrees += deg;
        }

        let mut theta = vec![active_squares as f64 / active_degrees as f64];

        // Perform the specified number of steps of the simulation.
        for t in 1..=steps {
            if active_degrees == 0 {
                return Err("All vertices have been deactivated");
            }

            let p = random.gen::<f64>();

            // Perform the vertex arrival event.
            if p <= model.pv {
                let v = hypergraph.add_vertex();
                let m = model.size(t);
                let mut e = fenwick.sample_many(m - 1, random);

                e.push(v);

                for &u in &e {
                    let deg = hypergraph.degree[u as usize];

                    fenwick.add(u as usize, 1);

                    active_degrees += 1;
                    active_squares += 2*deg + 1;
                }

                hypergraph.add_edge(e);
            }

            // Perform the edge arrival event.
            else if p <= model.pv + model.pe {
                let m = model.size(t);
                let e = fenwick.sample_many(m, random);

                for &u in &e {
                    let deg = hypergraph.degree[u as usize];

                    fenwick.add(u as usize, 1);

                    active_degrees += 1;
                    active_squares += 2*deg + 1;
                }

                hypergraph.add_edge(e);
            }

            // Perform the vertex deactivation event.
            else {
                let v = fenwick.sample_one(random);
                let deg = hypergraph.degree[v as usize];

                fenwick.set(v as usize, 0);

                active_degrees -= deg;
                active_squares -= deg * deg;
            }

            theta.push(active_squares as f64 / active_degrees as f64);
        }

        Ok(Simulation { model: *model, hypergraph, steps, theta })
    }
}