use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use crate::board::{Board, BoardSection, Effect, Location};
use crate::evolve::{Evolve, Lifespan};

/// Plant entity that has a limited lifespan
#[derive(Clone, Debug)]
pub struct Plant {
    pub age: i64,
    pub age_max: i64,
    pub health: i64,
    pub health_max: i64,
    pub kind: PlantKind,
    pub location: Location,
    pub messages: Vec<String>,
    pub offspring: Vec<Plant>,
    pub offspring_chance: f64,
    pub requirements: Requirements,
    pub size: i64,
    pub size_max: i64,
}

#[derive(Clone, Debug)]
pub struct Requirements {
    pub light: Effect,
    pub moisture: Effect,
}

impl Plant {
    pub fn new(kind: PlantKind, board: &Board) -> Plant {

        // determine age_max
        let age_max = match kind {
            PlantKind::Fern => 12,
            PlantKind::Tree => 80,
        };

        // determine health_max
        let health_max = match kind {
            PlantKind::Fern => 10,
            PlantKind::Tree => 18,
        };

        // determine offspring factor
        let offspring_chance = match kind {
            PlantKind::Fern => 0.2,
            PlantKind::Tree => 0.2,
        };

        // determine requirements based on kind
        let requirements = match kind {
            PlantKind::Fern => Requirements {
                light: Effect::Light(20),
                moisture: Effect::Moisture(2),
            },
            PlantKind::Tree => Requirements {
                light: Effect::Light(20),
                moisture: Effect::Moisture(4),
            },
        };

        // determine size based on kind
        let size_max = match kind {
            PlantKind::Fern => 8,
            PlantKind::Tree => 50,
        };

        // Plant object
        let plant = Plant {
            age: 0,
            age_max,
            health: 1,
            health_max,
            kind,
            location: Location::new_random(board.size),
            messages: Vec::new(),
            offspring: Vec::new(),
            offspring_chance,
            requirements,
            size: 1,
            size_max,
        };
        plant
    }

    pub fn summary(&self) -> String {
        format!("Plant {{ kind: {:?} age: {:?}/{:?}, health: {:?}/{:?}, size: {:?}/{:?} location: {:?}}}",
                self.kind,
                self.age,
                self.age_max,
                self.health,
                self.health_max,
                self.size,
                self.size_max,
                self.location,
        )
    }
}

#[derive(Clone, Debug)]
pub enum PlantKind {
    Fern,
    Tree,
}

impl Evolve for Plant {
    fn evolve(&mut self, section: &mut BoardSection) {
        // Save current state for comparison after evolution
        let previous = self.clone();
        let offspring = self.biology(section);

        // check for returned propagation
        match offspring {
            Some(offspring) => {
                for o in offspring {
                    self.offspring.push(o);
                }
            },
            None => (),
        }
        if self.health == 0 && previous.health != 0 {
            self.messages.push(format!("The {:?} perishes", self.kind));
        }
    }
}

impl Lifespan for Plant {
    fn alive(&self) -> bool {
        if self.health > 0 {
            return true;
        }
        false
    }

    fn biology(&mut self, section: &mut BoardSection) -> Option<Vec<Plant>> {
        self.age += 1;
        if self.alive() {
            // death upon exhaustion of lifespan
            if self.age > self.age_max {
                self.health = 0;
                // do not continue if we are dead
                return None;
            }

            // Respiration
            match self.requirements.moisture {
                Effect::Moisture(v) => {
                    if section.conditions.moisture >= v as i64 {
                        // consume moisture from section
                        section.conditions.moisture -= v as i64;
                        // TODO: grow at this juncture (or signal immediately)
                        self.grow();
                        // TODO: we should probably bind entities to a BoardSection
                        // then we can easily add plants from this scope.

                        // establish chance to propagate
                        let spawn_chance: f64 = rand::thread_rng().gen();
                        // if self.health == self.health_max {
                        // must be mature to reproduce
                        let size_percent =  self.size as f64 / self.size_max as f64;
                        if size_percent > 0.8 {
                            self.offspring = match spawn_chance {
                                chance if chance < self.offspring_chance => self.propagate(1),
                                _ => vec![],
                            }
                        }
                    } else {
                        // use all available moisture even though we take damage
                        section.conditions.moisture = 0;
                        self.damage(1);
                    }
                },
                _ => (),
            }
        }
        None
    }

    fn damage(&mut self, damage: i64) {
        self.health = match self.health.checked_sub(damage) {
            Some(v) => v,
            None => 0,
        }
    }

    fn grow(&mut self) {
        if self.health < self.health_max {
            self.health += 1;
        }
        if self.size < self.size_max {
            self.size += 1;
        }
    }

    /// Optionally spawns new plants in nearby coordinates.
    fn propagate(&mut self, num: i64) -> Vec<Plant> {
        // determine nearby location
        let mut rng = StdRng::from_entropy();
        let locations = self.location.nearby();
        let pick = rng.gen_range(0..locations.len());
        let location = locations[pick].clone();

        // create new seedling
        let sprout = Plant {
            age: 0,
            health: 1,
            health_max: self.health_max,
            kind: self.kind.clone(),
            // location: Location::new_random(self.location.max),
            location,
            age_max: self.age_max,
            messages: Vec::new(),
            offspring: Vec::new(),
            offspring_chance: self.offspring_chance,
            requirements: self.requirements.clone(),
            size: 1,
            size_max: self.size_max,
        };
        // change to spawn an extra offspring if health is at max
        let mut offspring: Vec<Plant> = Vec::new();
        for _ in 0..num as i64 {
            offspring.push(sprout.clone());
        }
        return offspring;
    }
}