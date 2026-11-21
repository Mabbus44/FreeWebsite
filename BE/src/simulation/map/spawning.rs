use rand::RngExt;
use super::entity::{EntityRef, EntityType};
use super::entity::carnivore::Carnivore;
use super::entity::herbivore::Herbivore;
use super::Map;

impl Map {
    pub(super) fn generate_entities(&mut self, entity_count: u32, entity_type: EntityType, from_best: bool) {
        for _ in 0..entity_count {
            let mut retries: u8 = 0;
            while retries < 3 {
                let pos_x = rand::rng().random_range(0..self.map_config.size_x);
                let pos_y = rand::rng().random_range(0..self.map_config.size_y);
                let mut is_free = false;
                match entity_type {
                    EntityType::Herbivore | EntityType::Carnivore => {
                        is_free = matches!(self.animals[pos_y][pos_x], EntityRef::None);
                    }
                    EntityType::Grass => {
                        is_free = matches!(self.plants[pos_y][pos_x], EntityRef::None);
                    }
                    _ => {}
                }
                if is_free {
                    match entity_type {
                        EntityType::Grass => {
                            self.plants[pos_y][pos_x] = EntityRef::Grass;
                            self.grass_count += 1;
                        }
                        EntityType::Herbivore => {
                            self.animals[pos_y][pos_x] = EntityRef::Herbivore(self.herbivores.len());
                            if from_best {
                                let best_id = rand::rng().random_range(0..self.best_herbivores.len());
                                self.herbivores.push(Some(Herbivore::new_from_parent(&self.map_config.mutation_config, &self.best_herbivores[best_id], self.map_config.herbivore_max_energy, pos_x, pos_y)));
                            } else {
                                self.herbivores.push(Some(Herbivore::new(self.input_count, self.map_config.neuron_count, self.map_config.neuron_layer_count, self.map_config.herbivore_max_energy, pos_x, pos_y)));
                            }
                            self.herbivore_count += 1;
                        }
                        EntityType::Carnivore => {
                            self.animals[pos_y][pos_x] = EntityRef::Carnivore(self.carnivores.len());
                            if from_best {
                                let best_id = rand::rng().random_range(0..self.best_carnivores.len());
                                self.carnivores.push(Some(Carnivore::new_from_parent(&self.map_config.mutation_config, &self.best_carnivores[best_id], self.map_config.carnivore_max_energy, pos_x, pos_y)));
                            } else {
                                self.carnivores.push(Some(Carnivore::new(self.input_count, self.map_config.neuron_count, self.map_config.neuron_layer_count, self.map_config.carnivore_max_energy, pos_x, pos_y)));
                            }
                            self.carnivore_count += 1;
                        }
                        _ => {}
                    }
                    break;
                }
                else {
                    retries += 1;
                }
            }
        }
    }
}