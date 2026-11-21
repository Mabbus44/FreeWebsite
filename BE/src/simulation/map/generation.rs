use crate::simulation::map::entity::{EntityRef, EntityType};
use crate::simulation::record::GenerationRecording;
use super::entity::animal::Animal;
use super::Map;

pub mod config;

pub fn move_best_animals<T: Animal>(
    animals: &mut Vec<Option<T>>,
    best_animals: &mut Vec<T>,
    animal_count: &mut u32, // Change to usize if your count field is usize
    max_best: u32,
) {
    let mut id = 0;
    while *animal_count > 0 && best_animals.len() < max_best as usize && id < animals.len() {
        if let Some(animal) = animals[id].take() {
            best_animals.push(animal);
            *animal_count -= 1;
        }
        id += 1;
    }
}

impl Map {
    pub fn start_new_generation(&mut self) {
        if !self.generation_over {
            panic!("Tried to start a new generation before the old one was over");
        }
        self.init_generation(true);
    }

    pub(super) fn init_generation(&mut self, is_subsequent_gen: bool) {
        self.tick = 0;
        self.carnivore_count = 0;
        self.herbivore_count = 0;
        self.grass_count = 0;
        self.generation_over = false;
        self.plants = vec![vec![EntityRef::None; self.map_config.size_x]; self.map_config.size_y];
        self.animals = vec![vec![EntityRef::None; self.map_config.size_x]; self.map_config.size_y];
        self.carnivores = Vec::with_capacity(self.map_config.carnivore_count as usize);
        self.herbivores = Vec::with_capacity(self.map_config.herbivore_count as usize);
        self.generate_entities(self.map_config.carnivore_count, EntityType::Carnivore, is_subsequent_gen);
        self.generate_entities(self.map_config.herbivore_count, EntityType::Herbivore, is_subsequent_gen);
        self.generate_entities(self.map_config.grass_count, EntityType::Grass, is_subsequent_gen);
        self.best_carnivores.clear();
        self.best_herbivores.clear();
        if self.map_config.record {
            self.recording = Some(GenerationRecording::new(self));
        }
    }

    pub fn set_generation_over(&mut self) -> bool {
        if self.generation_over {
            return true;
        }
        if self.map_config.generation_config.max_ticks_per_generation >= 0 && self.tick >= self.map_config.generation_config.max_ticks_per_generation as usize {
            self.generation_over = true;
        }
        if self.map_config.generation_config.all_entities_must_be_under_min_levels {
            if (self.map_config.generation_config.carnivore_count >= 0 && self.carnivore_count as i32 <= self.map_config.generation_config.carnivore_count) &&
                (self.map_config.generation_config.herbivore_count >= 0 && self.herbivore_count as i32 <= self.map_config.generation_config.herbivore_count) &&
                (self.map_config.generation_config.grass_count >= 0 && self.grass_count as i32 <= self.map_config.generation_config.grass_count) {
                self.generation_over = true;
            }
        } else {
            if (self.map_config.generation_config.carnivore_count >= 0 && self.carnivore_count as i32 <= self.map_config.generation_config.carnivore_count) ||
                (self.map_config.generation_config.herbivore_count >= 0 && self.herbivore_count as i32 <= self.map_config.generation_config.herbivore_count) ||
                (self.map_config.generation_config.grass_count >= 0 && self.grass_count as i32 <= self.map_config.generation_config.grass_count) {
                self.generation_over = true;
            }
        }
        if self.generation_over {
            self.move_remaining_animals_to_best();
        }
        return self.generation_over;
    }
}