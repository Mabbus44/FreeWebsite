use serde::Serialize;

pub mod herbivore;
pub mod carnivore;
pub mod animal;
pub mod config;
mod neuron;

#[derive(Clone)]
pub enum EntityRef {
    Herbivore(usize),
    Carnivore(usize),
    Grass,
    None
}

#[derive(Clone)]
pub enum EntityType {
    Grass,
    Herbivore,
    Carnivore,
    None
}

#[derive(Clone, Copy)]
#[derive(Serialize)]
pub enum Action {
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
    Eat,
    None
}

const ACTION_COUNT: usize = 6;
