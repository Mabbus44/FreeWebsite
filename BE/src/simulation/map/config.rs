use serde::Deserialize;
use super::entity::config::MutationConfig;
use super::generation::config::GenerationConfig;
#[derive(Clone)]
#[derive(Deserialize)]
pub struct MapConfig {
    pub carnivore_count: u32,
    pub herbivore_count: u32,
    pub grass_count: u32,
    pub neuron_count: usize,
    pub neuron_layer_count: usize,
    pub sense_radius: usize,
    pub carnivore_max_energy: u32,
    pub herbivore_max_energy: u32,
    pub size_x: usize,
    pub size_y: usize,
    pub record: bool,
    pub generation_config: GenerationConfig,
    pub mutation_config: MutationConfig
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            carnivore_count: 5,
            herbivore_count: 15,
            grass_count: 100,
            neuron_count: 10,
            neuron_layer_count: 2,
            sense_radius: 4,
            carnivore_max_energy: 100,
            herbivore_max_energy: 50,
            size_x: 200,
            size_y: 100,
            record: true,
            generation_config: GenerationConfig::default(),
            mutation_config: MutationConfig::default()
        }
    }
}