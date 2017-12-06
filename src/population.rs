use organism::Organism;
use species::Species;
use genome::Genome;
use std::vec::Vec;
use Mutator;

/// ---------------------------------------------
/// POPULATION STRUCT:
///   A Population is a group of Organisms
///   including their species
/// ---------------------------------------------
pub struct Population {
    /// The organisms in the Population
    organisms: Vec<Box<Organism>>,
    /// Species in the Population. Note that the species should comprise all the genomes
    species: Vec<Box<Species>>,

    // ******* Member variables used during reproduction *******
    ///// For holding the genetic innovations of the newest generation
    //innovations: std::Vec<Box<Innovations>>,
    /// Current label number available
    cur_node_id: u32,
    cur_innov_num: f64,
    last_species: u32,

    // ******* Fitness Statistics *******
    mean_fitness: f64,
    variance: f64,
    standard_deviation: f64,
    ///An integer that when above zero tells when the first winner appeared
    winnergen: i32,

    // ******* When do we need to delta code? *******
    ///Stagnation detector
    highest_fitness: f64,
    ///If too high, leads to delta coding
    highest_last_changed: u32,
}

impl Population {
    pub fn new(start_genome: &Genome, pop_size: usize) -> Self
    {
        let population = Population {
            organisms: Vec::new(),
            species: Vec::new(),
            cur_node_id: 0,
            cur_innov_num: 0.0,
            last_species: 0,
            mean_fitness: 0.0,
            variance: 0.0,
            standard_deviation: 0.0,
            winnergen: 0,
            highest_fitness: 0.0,
            highest_last_changed: 0,
        };

        for count in 0..pop_size {
            let mut new_genome = start_genome.clone(count as i32);

            new_genome.mutate_link_weights(1.0, 1.0, Mutator::ColdGaussian);
//            new_genome.randomize_traits();
//            let new_organism = Organism::new(0.0, new_genome, 1);
//            population.organisms.push(new_organism);
        }

        population
    }
}