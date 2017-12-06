extern crate rand;

#[cfg(test)]
mod tests;

pub mod env;
pub mod population;
pub mod species;
pub mod organism;
pub mod genome;
pub mod node;
pub mod link;
pub mod network;

pub enum Mutator {
    Gaussian,
    ColdGaussian,
}