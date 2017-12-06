pub mod gene;
pub mod gene_trait;

use self::gene_trait::Trait;
use self::gene::Gene;
use Mutator;
use env::Env;
use rand;
use rand::distributions::{IndependentSample, Range};
use node::Node;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Genome {
    id: i32,
    traits: Vec<Rc<RefCell<Trait>>>,
    nodes: Vec<Rc<RefCell<Node>>>,
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

    pub fn add_trait(&mut self, gene_trait: Rc<RefCell<Trait>>)
    {
        self.traits.push(gene_trait)
    }

    pub fn add_node(&mut self, node: Rc<RefCell<Node>>)
    {
        self.nodes.push(node)
    }

    pub fn add_gene(&mut self, gene: Gene)
    {
        self.genes.push(gene)
    }

    pub fn clone(&self, new_id: i32) -> Self {
        let mut traits_hash = HashMap::new();
        let mut nodes_hash = HashMap::new();

        let mut new_genome = Genome {
            id: new_id,
            traits: self.traits.clone(),
            nodes: Vec::new(),
            genes: Vec::new(),
        };

        for new_trait_ref in &self.traits {
            let new_trait = new_trait_ref.clone();
            let id = new_trait.borrow().id;
            traits_hash.insert(id, new_trait);
        }

        for node in &self.nodes {
            let mut new_node = node.borrow().duplicate();
            if let Some(old_nodetrait) = new_node.node_trait {
                new_node.node_trait = Some(traits_hash.get(&old_nodetrait.borrow().id).unwrap().clone())
            } else {
                new_node.node_trait = Some(new_genome.traits[0].clone())
            }
            let new_node_arc = Rc::new(RefCell::new(new_node));
            nodes_hash.insert(new_node_arc.borrow().id, new_node_arc.clone());
            new_genome.nodes.push(new_node_arc);
        }

        for gene in &self.genes {
            let mut new_gene = gene.clone();

            new_gene.link.i_node = nodes_hash.get(&gene.link.i_node.borrow().id).unwrap().clone();
            new_gene.link.o_node = nodes_hash.get(&gene.link.o_node.borrow().id).unwrap().clone();

            if let Some(ref link_trait_old) = gene.link.link_trait {
                new_gene.link.link_trait = Some(traits_hash.get(&link_trait_old.borrow().id).unwrap().clone());
            } else {
                new_gene.link.link_trait = Some(new_genome.traits[0].clone());
            }

            new_genome.genes.push(new_gene);
        }

        new_genome
    }

    pub fn mutate_link_weights(&mut self, power: f64, rate: f64, mutator_type: Mutator)
    {
        let severe_mutation = rand::random::<bool>();

        //Go through all the Genes and perturb their link's weights
        let mut num = 0.0;
        let gene_total = self.genes.len() as f64;
        let end_part = gene_total * 0.8;
        //let powermod = (if rand::random::<bool>() {1.0} else {-1.0})
        //    * power
        //    * rand::random::<f64>();  //Make power of mutation random
        //let powermod = rand::random::<f64>();
        let powermod = 1.0;

        //Loop on all genes  (ORIGINAL METHOD)
        for gene in &mut self.genes {
            //The following if determines the probabilities of doing cold gaussian
            //mutation, meaning the probability of replacing a link weight with
            //another, entirely random weight.  It is meant to bias such mutations
            //to the tail of a genome, because that is where less time-tested genes
            //reside.  The gausspoint and coldgausspoint represent values above
            //which a random float will signify that kind of mutation.

            //Don't mutate weights of frozen links
            if gene.is_frozen() {
                continue
            }

            let gauss_point;
            let cold_gauss_point;

            if severe_mutation {
                gauss_point = 0.3;
                cold_gauss_point = 0.1;
            } else if (gene_total >= 10.0) && (num > end_part) {
                gauss_point = 0.5; // Mutate by modification % of connections
                cold_gauss_point = 0.3; // Mutate the rest by replacement % of t6he time
            } else {
                gauss_point = 1.0 - rate;
                if rand::random::<bool>() {
                    cold_gauss_point = 1.0 - rate - 0.1;
                } else {
                    cold_gauss_point = 1.0 - rate;
                }
            }


            let random_num: f64 = (if rand::random::<bool>() { 1.0 } else { -1.0 })
                * rand::random::<f64>()
                * power
                * powermod;

            match mutator_type {
                Mutator::Gaussian => {
                    let random_choice = rand::random::<f64>();
                    if random_choice > gauss_point {
                        gene.link.weight += random_num;
                    } else if random_choice > cold_gauss_point {
                        gene.link.weight = random_num;
                    }
                }
                Mutator::ColdGaussian => {
                    gene.link.weight = random_num;
                }
            }


            // Cap the weights at 8.0 (experimental)
            if gene.link.weight > 8.0 {
                gene.link.weight = 8.0;
            } else if gene.link.weight < -8.0 {
                gene.link.weight = -8.0;
            }

            // Record the innovation
            gene.mutation_num = gene.link.weight;

            num += 1.0;
        }
    }

    pub fn randomize_traits(&mut self) {
        let num_traits = self.traits.len();
        let between: Range<usize> = Range::new(0, num_traits);
        let mut rng = rand::thread_rng();

        for node in &mut self.nodes {
            let trait_num = between.ind_sample(&mut rng);
            node.borrow_mut().node_trait = Some(self.traits[trait_num].clone());
        }

        for gene in &mut self.genes {
            let trait_num = between.ind_sample(&mut rng);
            gene.link.link_trait = Some(self.traits[trait_num].clone());
        }
    }

    pub fn get_last_node_id(&self) -> Option<i32>
    {
        self.nodes.last().map(|node| node.borrow().id)
    }

    pub fn get_last_gene_innovnum(&self) -> Option<f64>
    {
        self.genes.last().map(|gene| gene.get_innovation_num())
    }

    pub fn compatibility(&self, other: &Genome, env: &Env) -> f64
    {
        //Set up the counters
        let mut num_disjoint = 0.0;
        let mut num_excess = 0.0;
        let mut mut_diff_total = 0.0;
        let mut num_matching = 0.0;  //Used to normalize mutation_num differences

        //Get the length of the longest Genome for percentage computations
        let max_genome_size =
            if self.genes.len() > other.genes.len() {
                self.genes.len()
            } else {
                other.genes.len()
            } as f64;

        //Now move through the Genes of each potential parent
        //until both Genomes end

        let mut gene1_iter = self.genes.iter();
        let mut gene2_iter = other.genes.iter();

        let mut p1gene = gene1_iter.next();
        let mut p2gene = gene2_iter.next();

        while p1gene.is_some() || p2gene.is_some() {
            if p1gene.is_none() {
                p2gene = gene2_iter.next();
                num_excess += 1.0;
            } else if p2gene.is_none() {
                p1gene = gene1_iter.next();
                num_excess += 1.0;
            } else {
                let p1gene_unwrapped = p1gene.unwrap();
                let p2gene_unwrapped = p2gene.unwrap();


                let p1innov = p1gene_unwrapped.get_innovation_num();
                let p2innov = p2gene_unwrapped.get_innovation_num();

                if p1innov == p2innov {
                    let mut mut_diff = p1gene_unwrapped.mutation_num - p2gene_unwrapped.mutation_num;
                    if mut_diff < 0.0 {
                        mut_diff = 0.0 - mut_diff;
                    }
                    mut_diff_total += mut_diff;

                    p1gene = gene1_iter.next();
                    p2gene = gene2_iter.next();
                    num_matching += 1.0;
                } else if p1innov < p2innov {
                    p1gene = gene1_iter.next();
                    num_disjoint += 1.0;
                } else if p2innov < p1innov {
                    p2gene = gene2_iter.next();
                    num_disjoint += 1.0;
                }
            }
        }

        env.disjoint_coeff * (num_disjoint / 1.0)
            + env.excess_coeff * (num_excess / 1.0)
            + env.mutdiff_coeff * (mut_diff_total / num_matching)
    }

    pub fn verify(&self) -> Result<(), String>
    {
        //Check each gene's nodes
        for gene in &self.genes {
            let i_node = &gene.link.i_node;
            let o_node = &gene.link.o_node;

            let mut i_node_found = false;
            let mut o_node_found = false;

            // look for i_node and o_node
            for node in &self.nodes {
                if i_node == node {
                    i_node_found = true;
                } else if o_node == node {
                    o_node_found = true;
                }
            }

            if !i_node_found {
                return Err(format!("input node in link not found in node list for node: {:?}", gene));
            }
            if !o_node_found {
                return Err(format!("output node in link not found in node list for node: {:?}", gene));
            }
        }

        //Check for NNodes being out of order
        let mut last_id: i32 = 0;
        for node in &self.nodes {
            if node.borrow().id < last_id {
                return Err(format!("Nodes out of order!"));
            }
            last_id = node.borrow().id;
        }

        //Make sure there are no duplicate genes
        let mut counter_1: usize = 0;
        for gene in &self.genes {
            let mut counter_2: usize = 0;
            for gene2 in &self.genes {
                if counter_1 != counter_2 && gene == gene2 {
                    return Err(format!("Duplicated genes"));
                }
                counter_2 += 1;
            }
            counter_1 += 1;
        }


        Ok(())
    }
}