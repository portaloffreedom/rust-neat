pub mod gene;
pub mod gene_trait;

use self::gene_trait::Trait;
use self::gene::Gene;
use node::Node;
use std::sync::Arc;

pub struct Genome {
    id: i32,
    traits: Vec<Arc<Trait>>,
    nodes: Vec<Arc<Node>>,
    genes: Vec<Gene>,
}

impl Genome {
    pub fn new(id: i32) -> Self
    {
        Genome {
            id,
            traits: Vec::new(),
            nodes: Vec::new(),
            genes: Vec::new(),
        }
    }

    pub fn add_trait(&mut self, gene_trait: Arc<Trait>)
    {
        self.traits.push(gene_trait)
    }

    pub fn add_node(&mut self, node: Arc<Node>)
    {
        self.nodes.push(node)
    }

    pub fn add_gene(&mut self, gene: Gene)
    {
        self.genes.push(gene)
    }

    pub fn clone(&self, new_id: i32) -> Self {
        let mut new_genome = Genome {
            id: new_id,
            traits: self.traits.clone(),
            nodes: Vec::new(),
            genes: Vec::new(),
        };

        for node in &mut self.nodes {
            use std::borrow::Borrow;
            use std::borrow::ToOwned;

            let ref_node: Node = node.borrow().clone();
            let new_node = Arc::new(ref_node.clone());
            new_genome.nodes.push(new_node);
        }

        for gene in &mut self.genes {
            new_genome.genes.push(gene.clone());
        }

        new_genome
    }
}