pub mod entity;
pub mod generation;
pub mod config;
mod sensing;
mod spawning;

use entity::EntityType;
use entity::carnivore::Carnivore;
use entity::herbivore::Herbivore;
use entity::{animal::Animal, Action, EntityRef};
use std::{cmp, mem};
use super::record::GenerationRecording;
use config::MapConfig;

pub struct Map {
    animals: Vec<Vec<EntityRef>>,
    plants: Vec<Vec<EntityRef>>,
    carnivores: Vec<Option<Carnivore>>,
    herbivores: Vec<Option<Herbivore>>,
    best_carnivores: Vec<Carnivore>,
    best_herbivores: Vec<Herbivore>,
    carnivore_count: u32,
    herbivore_count: u32,
    grass_count: u32,
    tick: usize,
    generation_over: bool,
    input_count: usize,
    recording: Option<GenerationRecording>,
    map_config: MapConfig,
}

impl Map {
    pub fn new(map_config: MapConfig) -> Self {
        let mut map = Map {
            animals: Vec::new(),
            plants: Vec::new(),
            carnivores: Vec::new(),
            herbivores: Vec::new(),
            best_carnivores: Vec::new(),
            best_herbivores: Vec::new(),
            carnivore_count: 0,
            herbivore_count: 0,
            grass_count: 0,
            generation_over: false,
            // Carnivore/Herbivore/Plant/Wall each has an input_side x input_side square of inputs, the centers are not included since it is occupied by the animal itself
            input_count: ((map_config.sense_radius * 2 + 1) * (map_config.sense_radius * 2 + 1) - 1) * 4,
            tick: 0,
            recording: None,
            map_config
        };
        map.init_generation(false);
        map
    }

    pub fn get_carnivores(&self) -> &Vec::<Option<Carnivore>> {
        &self.carnivores
    }

    pub fn get_herbivores(&self) -> &Vec::<Option<Herbivore>> {
        &self.herbivores
    }

    pub fn get_grass_count(&self) -> u32 {
        self.grass_count
    }

    pub fn get_plants(&self) -> &Vec<Vec<EntityRef>> {
        &self.plants
    }

    fn move_remaining_animals_to_best(&mut self) {
        generation::move_best_animals(&mut self.herbivores, &mut self.best_herbivores, &mut self.herbivore_count, self.map_config.generation_config.best_herbivore_count);
        generation::move_best_animals(&mut self.carnivores, &mut self.best_carnivores, &mut self.carnivore_count, self.map_config.generation_config.best_carnivore_count);
        if self.best_carnivores.is_empty() || self.best_herbivores.is_empty() {
            panic!("Best carnivores or herbivores is empty");
        }
    }

    pub fn tick(&mut self) -> bool {
        self.calculate_actions();
        if self.map_config.record {
            self.recording.as_mut().unwrap().record_tick(&self.carnivores, &self.herbivores);
        }
        self.take_actions();
        self.tick += 1;
        self.set_generation_over()
    }

    #[allow(dead_code)]
    pub fn get_tick_count(&self) -> usize { self.tick }

    pub fn calculate_actions(&mut self) {
        Self::calculate_actions_for(&(self.animals), self.map_config.sense_radius, &mut self.carnivores);
        Self::calculate_actions_for(&(self.animals), self.map_config.sense_radius, &mut self.herbivores);
    }

    fn calculate_actions_for<A: Animal>(animals_grid: &Vec<Vec<EntityRef>>, sense_radius: usize, list: &mut [Option<A>]){
        for animal in list.iter_mut().flatten() {
            let (x, y) = animal.get_position();
            let input = sensing::generate_input(animals_grid, sense_radius, x as i32, y as i32);
            animal.calculate_action(input);
        }
    }

    fn take_actions(&mut self) {
        for i in 0..self.carnivores.len() {
            let (x, y, action) = match &self.carnivores[i] {
                Some(carnivore) => {
                    let (x, y) = carnivore.get_position();
                    (x, y, carnivore.get_action())
                }
                None => continue,
            };
            let success = self.take_action(EntityType::Herbivore, x, y, action);
            if !self.carnivores[i].as_mut().unwrap().take_action(success) {
                self.kill_animal_by_id(i, EntityType::Carnivore);
            }
        }
        for i in 0..self.herbivores.len() {
            let (x, y, action) = match &self.herbivores[i] {
                Some(herbivore) => {
                    let (x, y) = herbivore.get_position();
                    (x, y, herbivore.get_action())
                }
                None => continue,
            };
            let success = self.take_action(EntityType::Grass, x, y, action);
            if !self.herbivores[i].as_mut().unwrap().take_action(success) {
                self.kill_animal_by_id(i, EntityType::Herbivore);
            }
        }
    }

    // Return true = valid action, map part of action performed. False = invalid action, nothing done
    fn take_action(&mut self, food_type: EntityType, x_in: usize, y_in: usize, action: Action) -> bool {
        match action {
            Action::WalkDown => {
                if y_in == 0 {
                    return false;
                }
                self.move_animal(x_in, y_in, x_in, y_in - 1)
            }
            Action::WalkUp => {
                self.move_animal(x_in, y_in, x_in, y_in + 1)
            }
            Action::WalkLeft => {
                if x_in == 0 {
                    return false;
                }
                self.move_animal(x_in, y_in, x_in - 1, y_in)
            }
            Action::WalkRight => {
                self.move_animal(x_in, y_in, x_in + 1, y_in)
            }
            Action::Eat => {
                let eat_range: i32 = 2;
                let x_min = cmp::max(x_in as i32 - eat_range, 0) as usize;
                let x_max = cmp::min(x_in as i32 + eat_range + 1, self.map_config.size_x as i32) as usize;
                let y_min = cmp::max(y_in as i32 - eat_range, 0) as usize;
                let y_max = cmp::min(y_in as i32 + eat_range + 1, self.map_config.size_y as i32) as usize;
                for y in y_min..y_max {
                    for x in x_min..x_max {
                        if x == x_in as usize && y == y_in as usize {
                            continue;
                        }
                        if matches!(food_type, EntityType::Herbivore) && let EntityRef::Herbivore(_) = &self.animals[y][x] {
                            self.kill_animal_by_pos(x, y);
                            return true;
                        }
                        if matches!(food_type, EntityType::Grass) && matches!(self.plants[y][x], EntityRef::Grass) {
                            if self.map_config.record && let Some(recording) = self.recording.as_mut() {
                                recording.add_dead_grass(self.tick, x, y);
                            }
                            self.plants[y][x] = EntityRef::None;
                            self.grass_count -= 1;
                            return true;
                        }
                    }
                }
                false
            }
            Action::None => {
                true
            }
        }
    }

    pub fn move_animal(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if to_x >= self.map_config.size_x || to_y >= self.map_config.size_y ||
            !matches!(self.animals[to_y][to_x], EntityRef::None) || matches!(self.animals[from_y][from_x], EntityRef::None) {
            return false;
        }
        if let EntityRef::Carnivore(id) = self.animals[from_y][from_x] {
            self.carnivores[id].as_mut().unwrap().set_position(to_x, to_y);
        }
        if let EntityRef::Herbivore(id) = self.animals[from_y][from_x] {
            self.herbivores[id].as_mut().unwrap().set_position(to_x, to_y);
        }
        self.animals[to_y][to_x] = mem::replace(&mut self.animals[from_y][from_x], EntityRef::None);
        true
    }

    pub fn kill_animal_by_pos(&mut self, x: usize, y: usize) {
        match self.animals[y][x] {
            EntityRef::Herbivore(id) => {
                if self.herbivore_count <= self.map_config.generation_config.best_herbivore_count {
                    self.best_herbivores.push(self.herbivores[id].as_ref().unwrap().clone());
                }
                self.animals[y][x] = EntityRef::None;
                self.herbivores[id] = None;
                self.herbivore_count -= 1;
            }
            EntityRef::Carnivore(id) => {
                if self.carnivore_count <= self.map_config.generation_config.best_carnivore_count {
                    self.best_carnivores.push(self.carnivores[id].as_ref().unwrap().clone());
                }
                self.animals[y][x] = EntityRef::None;
                self.carnivores[id] = None;
                self.carnivore_count -= 1;
            }
            EntityRef::Grass => {panic!("Tried to kill grass at {}, {}", x, y); }
            EntityRef::None => {panic!("Tried to kill none at {}, {}", x, y);}
        }
    }

    pub fn kill_animal_by_id(&mut self, id: usize, entity_type: EntityType) {
        match entity_type {
            EntityType::Herbivore => {
                let (x, y) = self.herbivores[id].as_ref().unwrap().get_position();
                self.kill_animal_by_pos(x, y);
            }
            EntityType::Carnivore => {
                let (x, y) = self.carnivores[id].as_ref().unwrap().get_position();
                self.kill_animal_by_pos(x, y);
            }
            EntityType::Grass => {panic!("Invalid EntityType Grass in kill_animal_by_id")}
            EntityType::None => {panic!("Invalid EntityType None in kill_animal_by_id")}
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret : String = "".to_string();
        for y in 0..self.map_config.size_y {
            let mut row : String = "".to_string();
            for x in 0..self.map_config.size_x {
                match &self.animals[y][x] {
                    EntityRef::Carnivore(_) => {
                        row.push('X');
                    }
                    EntityRef::Herbivore(_) => {
                        row.push('o');
                    }
                    EntityRef::Grass => {}
                    EntityRef::None => {
                        if matches!(&self.plants[y][x], EntityRef::Grass) {
                            row.push('.');
                        } else {
                            row.push(' ');
                        }
                    }
                }
            }
            ret.push_str(&row);
            ret.push('\n');
        }
        ret
    }
    
    pub fn get_recording(&mut self) -> Option<GenerationRecording> {
        self.recording.take()
    }
}


