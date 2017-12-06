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
    pub organisms: Vec<Rc<RefCell<Organism>>>,
    /// Species in the Population. Note that the species should comprise all the genomes
    pub species: Vec<Rc<RefCell<Species>>>,

    // ******* Member variables used during reproduction *******
    ///// For holding the genetic innovations of the newest generation
    //innovations: std::Vec<Box<Innovations>>,
    /// Current label number available
    cur_node_id: i32,
    cur_innov_num: f64,
    last_species: usize,

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
            for current_species in self.species.iter_mut() {
                let comparison_organism = current_species.borrow().organisms.first().unwrap().clone();

                if organism.borrow().genome.compatibility(&comparison_organism.borrow().genome, env) < env.compat_threshold {
                    // Found compatible species, so add this organism to it
                    current_species.borrow_mut().add_organism(organism.clone());
                    organism.borrow_mut().set_species(current_species.clone());
                    // The search is over
                    break;
                }
            }

            if !organism.borrow().has_species() {
                counter += 1;
                let mut new_species = Species::new(counter);
                new_species.add_organism(organism.clone());
                let new_species = Rc::new(RefCell::new(new_species));
                organism.borrow_mut().set_species(new_species.clone());
                self.species.push(new_species);
            }
        }

        self.last_species = counter;
    }

    pub fn verify(&self) -> Result<(), String>
    {
        for ref organism in &self.organisms {
            organism.borrow().genome.verify()?;
        }

        Ok(())
    }

    pub fn epoch(&mut self, generation: usize, env: &Env)
    {
        let total_organisms = self.organisms.len();

        let mut sorted_species: Vec<Rc<RefCell<Species>>> = Vec::new();

        for species in &self.species {
            sorted_species.push(species.clone());
        }

        //Sort the Species by max fitness (Use an extra list to do this)
        //These need to use ORIGINAL fitness
        //sorted_species.qsort(order_species);
        sorted_species.sort_by(|a, b| {
            a.borrow().max_fitness
                .partial_cmp(&b.borrow().max_fitness)
                .unwrap()
                //.reverse()
        });

        //Flag the lowest performing species over age 20 every 30 generations
        //NOTE: THIS IS FOR COMPETITIVE COEVOLUTION STAGNATION DETECTION

        if generation % 30 == 0 {
            for species in &sorted_species {
                if species.borrow().age > 20 {
                    species.borrow_mut().set_to_obliterate();
                    break;
                }
            }
        }

        println!("Number of species: {}", self.species.len());
        println!("compat_treshold: {}", env.compat_threshold);

        // Use Species' ages to modify the objective fitness of organisms in other words,
        // make it more fair for younger species so they have a chance to take hold.
        // Also penalize stagnant species.
        // Then adjust the fitness using the species size to "share" fitness within a species.
        // Then, within each Species, mark for death those below survival_thresh*average.
        for species in &self.species {
            species.borrow_mut().adjust_fitness(env);
        }


        //Go through the organisms and add up their fitnesses to compute the
        //overall average
        let mut total_fitness = 0.0;
        for organism in &self.organisms {
            total_fitness += organism.borrow().fitness;
        }

        let overall_average: f64 = total_fitness / total_organisms as f64;
        println!("Generation {}: overall_average = {}", generation, overall_average);

        //Now compute expected number of offspring for each individual organism
        for organism in &self.organisms {
            let mut organism = organism.borrow_mut();
            organism.expected_offspring =
                organism.fitness / overall_average;
        }

        //Now add those offspring up within each Species to get the number of
        //offspring per Species
        let mut skim = 0.0;
        let mut total_expected: usize = 0;
        for species in &mut self.species {
            skim = species.borrow_mut().count_offspring(skim);
            total_expected += species.borrow().expected_offspring;
        }

        //Need to make up for lost foating point precision in offspring assignment
        //If we lost precision, give an extra baby to the best Species
        if total_expected < total_organisms {
            // Find the Species expecting the most
            let mut max_expected = 0;
            let mut final_expected = 0;
            let mut best_species: Rc<RefCell<Species>> = self.species[0].clone();
            for _species in &self.species {
                let species = _species.borrow();
                if species.expected_offspring >= max_expected {
                    max_expected = species.expected_offspring;
                    best_species = _species.clone();
                }
                final_expected += species.expected_offspring;
            }

            // Give the extra offspring to the best species
            best_species.borrow_mut().expected_offspring += 1;
            final_expected += 1;

            // If we still aren't at total, there is a problem
            // Note that this can happen if a stagnant Species
            // dominates the population and then gets killed off by its age
            // Then the whole population plummets in fitness
            // If the average fitness is allowed to hit 0, then we no longer have
            // an average we can use to assign offspring.\
            if final_expected < total_organisms {
                //println!("Population died!");
                for species in &self.species {
                    species.borrow_mut().expected_offspring = 0;
                }
                best_species.borrow_mut().expected_offspring = total_organisms;
            }
        }


        //Sort the Species by max fitness (Use an extra list to do this)
        //These need to use ORIGINAL fitness
        sorted_species.sort_by(|a,b| {
            let org_a = &a.borrow().organisms;
            let org_b = &b.borrow().organisms;
            let org_a_fitness = org_a.first().unwrap().borrow().orig_fitness;
            let org_b_fitness = org_b.first().unwrap().borrow().orig_fitness;
            {
                org_a_fitness.partial_cmp(&org_b_fitness)
                .unwrap()
            }
        });
    }
}
