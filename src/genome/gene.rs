use std::sync::Arc;
use link::Link;
use genome::gene_trait::Trait;
use node::Node;

#[derive(Clone)]
pub struct Gene {
    link: Link,

    innovation_num: f64,
    /// Used to see how much mutation has changed the link
    mutation_num: f64,

    /// When this is off the Gene is disabled
    enable: bool,
    /// When frozen, the link weight cannot be mutated
    frozen: bool,

}

impl Gene {
    pub fn new(trait_gene: Option<Arc<Trait>>, i_node: Arc<Node>, o_node: Arc<Node>, weight: f64, recurrent: bool, innovation_num: f64, mutation_num: f64, enable: bool) -> Self
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
}