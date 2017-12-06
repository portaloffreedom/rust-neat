use env::Env;
use population::Population;
use genome::Genome;
use genome::gene::Gene;
use genome::gene_trait::Trait;
use node::Node;
use node::{NodeType, NodePlace};
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn it_loads_env() {
    let env = Env::load_from_file("assets/test.ne", true).unwrap();
}

#[test]
fn xor_test() {
    println!("START XOR TEST");

    let GENERATIONS: usize = 100;

    let env = Env::load_from_file("assets/test.ne", true).unwrap();

    let id: u32;

    // outputstream
    let mut gen: u32;

    let mut evals: Vec<i32> = vec![0; env.num_runs];
    let mut genes: Vec<i32> = vec![0; env.num_runs];
    let mut nodes: Vec<i32> = vec![0; env.num_runs];
    let winnernum: u32;
    let winnergenes: u32;
    let winnernodes: u32;

    // For averaging
    let mut total_evals: u32 = 0;
    let mut total_genes: u32 = 0;
    let mut total_nodes: u32 = 0;
    let mut exp_count: u32;
    let mut samples: u32;

    let mut start_genome = Genome::new(1);

    let trait_1 = Rc::new(RefCell::new(Trait::new(1, [0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])));
    let trait_2 = Rc::new(RefCell::new(Trait::new(2, [0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])));
    let trait_3 = Rc::new(RefCell::new(Trait::new(3, [0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])));
    start_genome.add_trait(trait_1.clone());
    start_genome.add_trait(trait_2.clone());
    start_genome.add_trait(trait_3.clone());

    let node_1 = Rc::new(RefCell::new(Node::new(1, None, NodeType::Sensor, NodePlace::Bias)));
    let node_2 = Rc::new(RefCell::new(Node::new(2, None, NodeType::Sensor, NodePlace::Input)));
    let node_3 = Rc::new(RefCell::new(Node::new(3, None, NodeType::Sensor, NodePlace::Input)));
    let node_4 = Rc::new(RefCell::new(Node::new(4, None, NodeType::Neuron, NodePlace::Output)));
    start_genome.add_node(node_1.clone());
    start_genome.add_node(node_2.clone());
    start_genome.add_node(node_3.clone());
    start_genome.add_node(node_4.clone());

    start_genome.add_gene(Gene::new(Some(trait_1), node_1, node_4.clone(), 0.0, false, 1.0, 0.0, true));
    start_genome.add_gene(Gene::new(Some(trait_2), node_2, node_4.clone(), 0.0, false, 2.0, 0.0, true));
    start_genome.add_gene(Gene::new(Some(trait_3), node_3, node_4.clone(), 0.0, false, 3.0, 0.0, true));

    for exp_count in 0..env.num_runs {
        println!("Spawning Population off Genome2");

        let population = Population::new(&start_genome, env.pop_size, &env);

        println!("Verifying Spawned Pop");
        population.verify().unwrap();

        for gen in 1..GENERATIONS {
            println!("Epoch {}", gen);

            //This is how to make a custom filename
            let file_name = format!("gen_{}", gen);

            //Check for success
//            if xor_epoch(population,gen,file_name,winnernum,winnergenes,winnernodes) {
//                //Collect Stats on end of experiment
//                evals[expcount]=env.pop_size*(gen-1)+winnernum;
//                genes[expcount]=winnergenes;
//                nodes[expcount]=winnernodes;
//                break;
//            }
        }
    }
}

#[test]
fn test_rand() {
    use rand;
    const ROUNDS: usize = 1000;

    let mut min = 11.0;
    let mut max = -11.0;
    let mut average = 0.0;

    for _ in 0..ROUNDS {
        let random_value = rand::random::<f64>();
        println!("{}", random_value);
        average += random_value;

        if random_value > max {
            max = random_value;
        } else if random_value < min {
            min = random_value;
        }
    }

    average /= ROUNDS as f64;

    println!("Min {} Max {} Average {}", min, max, average);
}