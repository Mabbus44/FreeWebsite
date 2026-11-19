use serde::Deserialize;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct MutationConfig {
    pub mutation_chance: f64,
    pub max_mutation_amount: f64
}

impl Default for MutationConfig {
    fn default() -> Self {
        Self {
            mutation_chance: 0.01,
            max_mutation_amount: 0.3,
        }
    }
}