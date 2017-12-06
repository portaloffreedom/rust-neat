use organism::Organism;
use species::Species;
use genome::Genome;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use Mutator;
use env::Env;

/// ---------------------------------------------
/// POPULATION STRUCT:
///   A Population is a group of Organisms
///   including their species
/// ---------------------------------------------
pub struct Population {
    /// The organisms in the Population
    organisms: Vec<Rc<RefCell<Organism>>>,
    /// Species in the Population. Note that the species should comprise all the genomes
    species: Vec<Rc<Species>>,

    // ******* Member variables used during reproduction *******
    ///// For holding the genetic innovations of the newest generation
    //innovations: std::Vec<Box<Innovations>>,
    /// Current label number available
    cur_node_id: i32,
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
    pub fn new(start_genome: &Genome, pop_size: usize, env: &Env) -> Self
    {
        let mut population = Population {
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
            let mut new_genome = Box::new(start_genome.clone(count as i32));

            new_genome.mutate_link_weights(1.0, 1.0, Mutator::ColdGaussian);
            new_genome.randomize_traits();
            let new_organism = Rc::new(RefCell::new(Organism::new(0.0, new_genome, 1)));
            population.organisms.push(new_organism);
        }

        population.cur_node_id = population.organisms.last().unwrap().borrow_mut().genome
            .get_last_node_id().unwrap();
        population.cur_innov_num = population.organisms.last().unwrap().borrow_mut().genome
            .get_last_gene_innovnum().unwrap();

        population.speciate(env);

        population
    }

    pub fn speciate(&mut self, env: &Env)
    {
        // Species counter
        let mut counter: usize = 0;

        for ref mut organism in &self.organisms {
            if self.species.len() == 0 {
                //Create the first species
                counter += 1;
                let mut new_species = Species::new(counter);
                new_species.add_organism(organism.clone()); // add current organism
                let new_species = Rc::new(new_species);
                organism.borrow_mut().set_species(new_species.clone());
                self.species.push(new_species); //
            } else {
                let mut current_species_iter = self.species.iter();
                let comparison_organism = self.species.first().unwrap().organisms.first().unwrap();

                while let Some(current_species) = current_species_iter.next() {
                    if organism.borrow().genome.compatibility(&comparison_organism.borrow().genome, env) < env.compat_threshold {

                    }
                }
            }
        }
    }
}