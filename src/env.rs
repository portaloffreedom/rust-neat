use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result as io_Result;
use std::io::Error as io_Error;
use std::io::ErrorKind as io_ErrorKind;

pub struct Env {
    pub trait_param_mut_prob: f64,
    // Power of mutation on a single trait param
    pub trait_mutation_power: f64,
    // Amount that mutation_num changes for a trait change inside a link
    pub linktrait_mut_sig: f64,
    // Amount a mutation_num changes on a link connecting a node that changed its trait
    pub nodetrait_mut_sig: f64,
    // The power of a linkweight mutation
    pub weight_mut_power: f64,
    // Prob. that a link mutation which doesn't have to be recurrent will be made recurrent
    pub recur_prob: f64,

    // These 3 global coefficients are used to determine the formula for
    // computating the compatibility between 2 genomes.  The formula is:
    // disjoint_coeff*pdg+excess_coeff*peg+mutdiff_coeff*mdmg.
    // See the compatibility method in the Genome class for more info
    // They can be thought of as the importance of disjoint Genes,
    // excess Genes, and parametric difference between Genes of the
    // same function, respectively.
    pub disjoint_coeff: f64,
    pub excess_coeff: f64,
    pub mutdiff_coeff: f64,

    // This global tells compatibility threshold under which two Genomes are considered the same species
    pub compat_threshold: f64,

    // Globals involved in the epoch cycle - mating, reproduction, etc..
    // How much does age matter?
    pub age_significance: f64,
    // Percent of ave fitness for survival
    pub survival_thresh: f64,
    // Prob. of a non-mating reproduction
    pub mutate_only_prob: f64,
    pub mutate_random_trait_prob: f64,
    pub mutate_link_trait_prob: f64,
    pub mutate_node_trait_prob: f64,
    pub mutate_link_weights_prob: f64,
    pub mutate_toggle_enable_prob: f64,
    pub mutate_gene_reenable_prob: f64,
    pub mutate_add_node_prob: f64,
    pub mutate_add_link_prob: f64,
    // Prob. of a mate being outside species
    pub interspecies_mate_rate: f64,
    pub mate_multipoint_prob: f64,
    pub mate_multipoint_avg_prob: f64,
    pub mate_singlepoint_prob: f64,
    // Prob. of mating without mutation
    pub mate_only_prob: f64,
    // Probability of forcing selection of ONLY links that are naturally recurrent
    pub recur_only_prob: f64,
    // Size of population
    pub pop_size: usize,
    // Age where Species starts to be penalized
    pub dropoff_age: usize,
    // Number of tries mutate_add_link will attempt to find an open link
    pub newlink_tries: u32,
    // Tells to print population to file every n generations
    pub print_every: usize,
    // The number of babies to siphon off to the champions
    pub babies_stolen: u32,

    //number of times to run experiment
    pub num_runs: usize,
}

impl Env {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            trait_param_mut_prob: 0.0,
            trait_mutation_power: 0.0,
            linktrait_mut_sig: 0.0,
            nodetrait_mut_sig: 0.0,
            weight_mut_power: 0.0,
            recur_prob: 0.0,
            disjoint_coeff: 0.0,
            excess_coeff: 0.0,
            mutdiff_coeff: 0.0,
            compat_threshold: 0.0,
            age_significance: 0.0,
            survival_thresh: 0.0,
            mutate_only_prob: 0.0,
            mutate_random_trait_prob: 0.0,
            mutate_link_trait_prob: 0.0,
            mutate_node_trait_prob: 0.0,
            mutate_link_weights_prob: 0.0,
            mutate_toggle_enable_prob: 0.0,
            mutate_gene_reenable_prob: 0.0,
            mutate_add_node_prob: 0.0,
            mutate_add_link_prob: 0.0,
            interspecies_mate_rate: 0.0,
            mate_multipoint_prob: 0.0,
            mate_multipoint_avg_prob: 0.0,
            mate_singlepoint_prob: 0.0,
            mate_only_prob: 0.0,
            recur_only_prob: 0.0,
            pop_size: 0,
            dropoff_age: 0,
            newlink_tries: 0,
            print_every: 0,
            babies_stolen: 0,
            num_runs: 0,
        })
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P, output: bool) -> io_Result<Box<Self>> {
        let mut env = Self::new();

        let f = File::open(path)?;
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = line?;
            if output { println!("{}", l); }

            let mut line_iterator = l.split_whitespace();

            if let Some(name) = line_iterator.next() {
                if let Some(value) = line_iterator.next() {
                    match name {
                        "trait_param_mut_prob" => env.trait_param_mut_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value trait_param_mut_prob: {}", e)))?,
                        "trait_mutation_power" => env.trait_mutation_power = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value trait_mutation_power: {}", e)))?,
                        "linktrait_mut_sig" => env.linktrait_mut_sig = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value linktrait_mut_sig: {}", e)))?,
                        "nodetrait_mut_sig" => env.nodetrait_mut_sig = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value nodetrait_mut_sig: {}", e)))?,
                        "weight_mut_power" => env.weight_mut_power = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value weight_mut_power: {}", e)))?,
                        "recur_prob" => env.recur_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value recur_prob: {}", e)))?,
                        "disjoint_coeff" => env.disjoint_coeff = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value disjoint_coeff: {}", e)))?,
                        "excess_coeff" => env.excess_coeff = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value excess_coeff: {}", e)))?,
                        "mutdiff_coeff" => env.mutdiff_coeff = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutdiff_coeff: {}", e)))?,
                        "compat_threshold" => env.compat_threshold = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value compat_threshold: {}", e)))?,
                        "age_significance" => env.age_significance = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value age_significance: {}", e)))?,
                        "survival_thresh" => env.survival_thresh = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value survival_thresh: {}", e)))?,
                        "mutate_only_prob" => env.mutate_only_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_only_prob: {}", e)))?,
                        "mutate_random_trait_prob" => env.mutate_random_trait_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_random_trait_prob: {}", e)))?,
                        "mutate_link_trait_prob" => env.mutate_link_trait_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_link_trait_prob: {}", e)))?,
                        "mutate_node_trait_prob" => env.mutate_node_trait_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_node_trait_prob: {}", e)))?,
                        "mutate_link_weights_prob" => env.mutate_link_weights_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_link_weights_prob: {}", e)))?,
                        "mutate_toggle_enable_prob" => env.mutate_toggle_enable_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_toggle_enable_prob: {}", e)))?,
                        "mutate_gene_reenable_prob" => env.mutate_gene_reenable_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_gene_reenable_prob: {}", e)))?,
                        "mutate_add_node_prob" => env.mutate_add_node_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_add_node_prob: {}", e)))?,
                        "mutate_add_link_prob" => env.mutate_add_link_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mutate_add_link_prob: {}", e)))?,
                        "interspecies_mate_rate" => env.interspecies_mate_rate = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value interspecies_mate_rate: {}", e)))?,
                        "mate_multipoint_prob" => env.mate_multipoint_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mate_multipoint_prob: {}", e)))?,
                        "mate_multipoint_avg_prob" => env.mate_multipoint_avg_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mate_multipoint_avg_prob: {}", e)))?,
                        "mate_singlepoint_prob" => env.mate_singlepoint_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mate_singlepoint_prob: {}", e)))?,
                        "mate_only_prob" => env.mate_only_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value mate_only_prob: {}", e)))?,
                        "recur_only_prob" => env.recur_only_prob = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value recur_only_prob: {}", e)))?,
                        "pop_size" => env.pop_size = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value pop_size: {}", e)))?,
                        "dropoff_age" => env.dropoff_age = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value dropoff_age: {}", e)))?,
                        "newlink_tries" => env.newlink_tries = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value newlink_tries: {}", e)))?,
                        "print_every" => env.print_every = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value print_every: {}", e)))?,
                        "babies_stolen" => env.babies_stolen = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value babies_stolen: {}", e)))?,
                        "num_runs" => env.num_runs = value.parse().map_err(|e| io_Error::new(io_ErrorKind::Other, format!("Error reading value num_runs: {}", e)))?,
                        _ => println!("WARNING! Env variable ({}) not recognized!", name),
                    }
                } else {
                    return Err(io_Error::new(io_ErrorKind::Other, "NEAT environment file not formatted correctly"));
                };
            }

            assert_eq!(None, line_iterator.next());
        }
        Ok(env)
    }
}
