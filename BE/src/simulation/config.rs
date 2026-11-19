use serde::Deserialize;
use super::map::config::MapConfig;
#[derive(Clone)]
#[derive(Deserialize)]
pub struct SimulationConfig {
    pub map_config: MapConfig,
    pub max_generation_count: u32
}

impl Default for SimulationConfig  {
    fn default() -> Self {
        Self {
            map_config: MapConfig::default(),
            max_generation_count: 50
        }
    }
}