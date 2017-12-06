use std::rc::Rc;
use std::cell::RefCell;
use link::Link;
use genome::gene_trait::Trait;
use node::Node;

#[derive(Clone)]
pub struct Gene {
    pub link: Link,

    innovation_num: f64,
    /// Used to see how much mutation has changed the link
    pub mutation_num: f64,

    /// When this is off the Gene is disabled
    enable: bool,
    /// When frozen, the link weight cannot be mutated
    frozen: bool,

}

impl Gene {
    pub fn new(trait_gene: Option<Rc<RefCell<Trait>>>, i_node: Rc<RefCell<Node>>, o_node: Rc<RefCell<Node>>, weight: f64, recurrent: bool, innovation_num: f64, mutation_num: f64, enable: bool) -> Self
    {
        let link = Link::new(trait_gene, weight, i_node, o_node, recurrent);


        Gene {
            link,
            innovation_num,
            mutation_num,
            enable,
            frozen: false,
        }
    }

    pub fn is_frozen(&self) -> bool { self.frozen }

    pub fn get_innovation_num(&self) -> f64 { self.innovation_num }
}