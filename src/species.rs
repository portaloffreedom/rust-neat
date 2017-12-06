use std::rc::Rc;
use std::cell::RefCell;
use organism::Organism;

pub struct Species {
    id: usize,
    pub organisms: Vec<Rc<RefCell<Organism>>>,
}

impl Species {
    pub fn new(id: usize) -> Self
    {
        Species {
            id,
            organisms: Vec::new(),
        }
    }

    pub fn add_organism(&mut self, organism: Rc<RefCell<Organism>>)
    {
        self.organisms.push(organism)
    }
}