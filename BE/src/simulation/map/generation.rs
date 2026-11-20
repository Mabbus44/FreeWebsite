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
        self.tick = 0;
        self.carnivore_count = 0;
        self.herbivore_count = 0;
        self.grass_count = 0;
        self.generation_over = false;
        self.plants = vec![vec![EntityRef::None; self.size_x]; self.size_y];
        self.animals = vec![vec![EntityRef::None; self.size_x]; self.size_y];
        self.carnivores = Vec::with_capacity(self.carnivore_start_count as usize);
        self.herbivores = Vec::with_capacity(self.herbivore_start_count as usize);
        self.generate_entities(self.carnivore_start_count, EntityType::Carnivore, true);
        self.generate_entities(self.herbivore_start_count, EntityType::Herbivore, true);
        self.generate_entities(self.grass_start_count, EntityType::Grass, true);
        self.best_carnivores = Vec::new();
        self.best_herbivores = Vec::new();
        if self.record {
            self.recording = Some(GenerationRecording::new(self));
        }
    }

    pub fn set_generation_over(&mut self) -> bool {
        if self.generation_over {
            return true;
        }
        if self.generation_config.max_ticks_per_generation >= 0 && self.tick >= self.generation_config.max_ticks_per_generation as usize {
            self.generation_over = true;
        }
        if self.generation_config.all_entities_must_be_under_min_levels {
            if (self.generation_config.carnivore_count >= 0 && self.carnivore_count as i32 <= self.generation_config.carnivore_count) &&
                (self.generation_config.herbivore_count >= 0 && self.herbivore_count as i32 <= self.generation_config.herbivore_count) &&
                (self.generation_config.grass_count >= 0 && self.grass_count as i32 <= self.generation_config.grass_count) {
                self.generation_over = true;
            }
        } else {
            if (self.generation_config.carnivore_count >= 0 && self.carnivore_count as i32 <= self.generation_config.carnivore_count) ||
                (self.generation_config.herbivore_count >= 0 && self.herbivore_count as i32 <= self.generation_config.herbivore_count) ||
                (self.generation_config.grass_count >= 0 && self.grass_count as i32 <= self.generation_config.grass_count) {
                self.generation_over = true;
            }
        }
        if self.generation_over {
            self.move_remaining_animals_to_best();
        }
        return self.generation_over;
    }
}