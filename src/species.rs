use std::rc::Rc;
use std::cell::RefCell;
use organism::Organism;
use env::Env;

pub struct Species {
    id: usize,
    pub organisms: Vec<Rc<RefCell<Organism>>>,
    pub average_fitness: f64,
    pub max_fitness: f64,
    pub max_fitness_ever: f64,
    pub age: usize,
    pub age_of_last_improvement: usize,
    pub expected_offspring: usize,
    obliterate: bool,
}

impl Species {
    pub fn new(id: usize) -> Self
    {
        Species {
            id,
            organisms: Vec::new(),
            average_fitness: 0.0,
            max_fitness: 0.0,
            max_fitness_ever: 0.0,
            age: 0,
            age_of_last_improvement: 0,
            expected_offspring: 0,
            obliterate: false,
        }
    }

    pub fn add_organism(&mut self, organism: Rc<RefCell<Organism>>)
    {
        self.organisms.push(organism)
    }

    pub fn compute_max_and_average_fitness(&mut self)
                                           -> (f64, f64)
    {
        let mut total = 0.0;
        let mut max = 0.0;

        for organism in &self.organisms {
            let organism_fitness = organism.borrow().get_fitness();
            total += organism_fitness;
            if organism_fitness > max {
                max = organism_fitness;
            }
        }

        self.average_fitness = total / (self.organisms.len() as f64);
        self.max_fitness = max;

        (self.average_fitness, max)
    }

    pub fn set_to_obliterate(&mut self) { self.obliterate = true }
    pub fn is_to_obliterate(&self) -> bool { self.obliterate }

    pub fn adjust_fitness(&mut self, env: &Env)
    {
        let mut age_debt: i32 = (self.age as i32 - self.age_of_last_improvement as i32 + 1) - env.dropoff_age as i32;
        if age_debt == 0 { age_debt = 1; }
        let organism_n = { self.organisms.len() };

        for organism in &self.organisms {
            let mut organism = organism.borrow_mut();

            //Remember the original fitness before it gets modified
            organism.orig_fitness = organism.fitness;

            //Make fitness decrease after a stagnation point dropoff_age
            //Added an if to keep species pristine until the dropoff point
            //obliterate is used in competitive coevolution to mark stagnation
            //by obliterating the worst species over a certain age
            if age_debt >= 1 || self.obliterate {
                //Possible graded dropoff
                //organism.fitness=organism.fitness*(-atan(age_debt));

                //Extreme penalty for a long period of stagnation (divide fitness by 100)
                organism.fitness = organism.fitness * 0.01;
            }

            // Give a fitness boost up to some young age (niching)
            // The age_significance parameter is a system parameter
            // if it is 1, then young species get no fitness boost
            if self.age <= 10 {
                organism.fitness = organism.fitness * env.age_significance;
            }

            //Do not allow negative fitness
            if organism.fitness < 0.0 {
                organism.fitness = 0.0001;
            }

            //Share fitness with the species
            organism.fitness = organism.fitness / (organism_n as f64);
        }

        //Sort the population and mark for death those after survival_thresh*pop_size
        //organisms.qsort(order_orgs);
        self.organisms.sort_by(
            |a, b| a.borrow().fitness.partial_cmp(&b.borrow().fitness).unwrap()
        );


        //Update age_of_last_improvement here
        if self.organisms.first().unwrap().borrow().orig_fitness > self.max_fitness_ever {
            self.age_of_last_improvement = self.age;
            self.max_fitness_ever = self.organisms.first().unwrap().borrow().orig_fitness;
        }

        //Decide how many get to reproduce based on survival_thresh*pop_size
        //Adding 1.0 ensures that at least one will survive
        let num_parents = ((env.survival_thresh * (self.organisms.len() as f64)) + 1.0).floor() as usize;

        //Mark the champ as such
        self.organisms.first().unwrap().borrow_mut().set_champion(true);

        let mut i: usize = 0;
        for organism in &self.organisms {
            i += 1;
            if i > num_parents {
                organism.borrow_mut().set_elimination(true);
            }
        }
    }

    pub fn count_offspring(&mut self, mut skim: f64) -> f64
    {
        self.expected_offspring = 0;

        for organism in &self.organisms {
            let e_o_intpart = organism.borrow().expected_offspring.floor() as usize;
            let e_o_fracpart = organism.borrow().expected_offspring.fract();

            self.expected_offspring += e_o_intpart;

            // Skim off the fractional offspring
            skim += e_o_fracpart;

            // NOTE: Some precision is lost by computer
            //       Must be remedied later
            if skim > 1.0 {
                let skim_intpart = skim.floor() as usize;
                self.expected_offspring += skim_intpart;
                skim = skim.fract();
            }
        }

        skim
    }
}