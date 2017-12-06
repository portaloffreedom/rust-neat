use env::Env;
use population::Population;
use genome::Genome;
use genome::gene::Gene;
use genome::gene_trait::Trait;
use node::Node;
use node::{NodeType, NodePlace};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;
use organism::Organism;

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
    let mut genes: Vec<usize> = vec![0; env.num_runs];
    let mut nodes: Vec<usize> = vec![0; env.num_runs];

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

        let mut population = Population::new(&start_genome, env.pop_size, &env);

        println!("Verifying Spawned Pop");
        population.verify().unwrap();

        for generation in 1..GENERATIONS {
            println!("Epoch {}", generation);

            //This is how to make a custom filename
            let file_name = format!("gen_{}", generation);

            //Check for success
            if let Ok((winner_num, winner_genes, winner_nodes)) = xor_epoch(&mut population, generation, file_name, &env) {
                //Collect Stats on end of experiment
                evals[exp_count] = (env.pop_size as i64 * (generation as i64 - 1) + winner_num as i64) as i32;
                genes[exp_count] = winner_genes;
                nodes[exp_count] = winner_nodes;
                break;
            }
        }
    }
}

fn xor_epoch<P: AsRef<Path>>(population: &mut Population, generation: usize, file_name: P, env: &Env)
                             -> Result<(i32, usize, usize), ()>
{
    let mut win = false;
    let mut winner_num: i32 = 0;
    let mut winner_genes: usize = 0;
    let mut winner_nodes: usize = 0;

    for organism in &mut population.organisms {
        if xor_evaluate(organism).is_ok() {
            win = true;
            winner_num = organism.borrow().genome.id;
            winner_genes = organism.borrow().genome.extrons();
            winner_nodes = organism.borrow().genome.nodes_n();
        }
    }

    for species in &population.species {
        species.borrow_mut().compute_max_and_average_fitness();
    }

    if win || env.print_every % generation == 0 {
        //population.print_to_file_by_species(filename);
        println!("TODO print on file by species");
    }

    population.epoch(generation, env);

    if win {
        for organism in &population.organisms {
            if organism.borrow().is_winner() {
                println!("WINNER IS #{}", organism.borrow().genome.id);
                organism.borrow().genome.print_to_file("xor_winner").unwrap();
            }
        }

        Ok((winner_num, winner_genes, winner_nodes))
    } else {
        Err(())
    }
}

/// Returns the fitness of the winner or the loser
fn xor_evaluate(organism: &mut Rc<RefCell<Organism>>)
                -> Result<f64, f64>
{
    //TODO evaluate XOR
    Ok(0.0001)
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
