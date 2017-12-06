use network::Network;
use genome::Genome;
use species::Species;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Organism {
    ///A measure of fitness for the Organism
    fitness: f64,
    ///A fitness measure that won't change during adjustments
    orig_fitness: f64,
    ///Used just for reporting purposes
    error: f64,
    ///Win marker (if needed for a particular task)
    winner: bool,
    ///The Organism's phenotype
    network: Network,
    ///The Organism's genotype
    pub genome: Box<Genome>,
    ///The Organism's Species
    species: Option<Rc<RefCell<Species>>>,
    ///Number of children this Organism may have
    expected_offspring: f64,
    ///Tells which generation this Organism is from
    generation: usize,
    ///Marker for destruction of inferior Organisms
    eliminate: bool,
    ///Marks the species champ
    champion: bool,
    ///Number of reserved offspring for a population leader
    super_champ_offspring: i32,
    ///Marks the best in population
    pop_champ: bool,
    ///Marks the duplicate child of a champion (for tracking purposes)
    pop_champ_child: bool,
    ///DEBUG variable- high fitness of champ
    high_fit: f64,
    ///When playing in real-time allows knowing the maturity of an individual
    time_alive: i32,

    // Track its origin -for debugging or analysis- we can tell how the organism was born
    mut_struct_baby: bool,
    mate_baby: bool,
    modified: bool,
}

impl Organism {
    pub fn new(fitness: f64, genome: Box<Genome>, generation: usize) -> Self
    {
        Organism {
            fitness,
            orig_fitness: fitness,
            error: 0.0,
            winner: false,
            network: Network::new(),
            genome,
            species: None,
            expected_offspring: 0.0,
            generation,
            eliminate: false,
            champion: false,
            super_champ_offspring: 0,
            pop_champ: false,
            pop_champ_child: false,
            high_fit: 0.0,
            time_alive: 0,
            mut_struct_baby: false,
            mate_baby: false,
            modified: true,
        }
    }

    pub fn set_species(&mut self, species: Rc<RefCell<Species>>)
    {
        self.species = Some(species);
    }

    pub fn has_species(&self) -> bool
    {
        self.species.is_some()
    }

    pub fn get_fitness(&self) -> f64
    {
        self.fitness
    }

    pub fn is_winner(&self) -> bool
    {
        self.winner
    }
}
