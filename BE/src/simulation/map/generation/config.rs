use serde::Deserialize;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct GenerationConfig {
    pub max_ticks_per_generation: i32,
    pub carnivore_count: i32,
    pub herbivore_count: i32,
    pub grass_count: i32,
    pub best_carnivore_count: u32,
    pub best_herbivore_count: u32,
    pub all_entities_must_be_under_min_levels: bool,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            max_ticks_per_generation: 1000,
            carnivore_count: 2,
            herbivore_count: 2,
            grass_count: -1,
            best_carnivore_count: 2,
            best_herbivore_count: 2,
            all_entities_must_be_under_min_levels: true,
        }
    }
}