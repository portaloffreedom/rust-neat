use std::rc::Rc;
use std::cell::RefCell;
use organism::Organism;

pub struct Species {
    id: usize,
    pub organisms: Vec<Rc<RefCell<Organism>>>,
    average_fitness: f64,
    max_fitness: f64,
}

impl Species {
    pub fn new(id: usize) -> Self
    {
        Species {
            id,
            organisms: Vec::new(),
            average_fitness: 0.0,
            max_fitness: 0.0,
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
}