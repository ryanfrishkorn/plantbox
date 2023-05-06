use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};

use crate::board::{Board, BoardSection, Effect, Location};
use crate::evolve::{Evolve, Lifespan};

/// Plant entity that has a limited lifespan
#[derive(Clone, Debug)]
pub struct Plant {
    pub age: i64,
    pub age_max: i64,
    pub flammability_chance: f64,
    pub health: i64,
    pub health_max: i64,
    pub kind: PlantKind,
    pub location: Location,
    pub messages: Vec<String>,
    pub offspring: Vec<Plant>,
    pub offspring_chance: f64,
    pub offspring_range: i64,
    pub on_fire: bool,
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
        // FIXME - this should be moved to proper logic, struct, or trait
        // determine age_max
        let age_max = match kind {
            PlantKind::Fern => 12,
            PlantKind::Tree => 80,
        };

        // will it burn?
        let flammability_chance = match kind {
            PlantKind::Fern => 0.99996,
            PlantKind::Tree => 0.99999,
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

        let offspring_range = match kind {
            PlantKind::Fern => 1,
            PlantKind::Tree => 3,
        };

        let on_fire = false;

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
        Plant {
            age: 0,
            age_max,
            flammability_chance,
            on_fire,
            health: 1,
            health_max,
            kind,
            location: Location::new_random(board.size),
            messages: Vec::new(),
            offspring: Vec::new(),
            offspring_chance,
            offspring_range,
            requirements,
            size: 1,
            size_max,
        }
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

impl PlantKind {
    pub fn icon(&self) -> char {
        match self {
            PlantKind::Fern => '🌿',
            PlantKind::Tree => '🌲',
        }
    }
}

impl Evolve for Plant {
    fn evolve(&mut self, section: &mut BoardSection) {
        // Save current state for comparison after evolution
        let previous = self.clone();
        let offspring = self.biology(section);

        // check for returned propagation
        if let Some(offspring) = offspring {
            for o in offspring {
                self.offspring.push(o);
            }
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

            // Burn her anyway!
            if self.on_fire {
                let calc_damage_rand: f64 = rand::thread_rng().gen();
                let calc_damage = (self.health_max as f64 * calc_damage_rand) * 0.1;
                self.damage(calc_damage as i64);
                if !self.alive() {
                    return None;
                }
            }

            // Respiration
            if let Effect::Moisture(v) = self.requirements.moisture {
                if section.conditions.moisture >= v && !self.on_fire {
                    // consume moisture from section
                    section.conditions.moisture -= v;
                    // TODO: grow at this juncture (or signal immediately)
                    self.grow();
                    // TODO: we should probably bind entities to a BoardSection
                    // then we can easily add plants from this scope.

                    // establish chance to propagate
                    let spawn_chance: f64 = rand::thread_rng().gen();
                    // if self.health == self.health_max {
                    // must be mature to reproduce
                    let size_percent = self.size as f64 / self.size_max as f64;
                    if size_percent > 0.8 {
                        self.offspring = match spawn_chance {
                            chance if chance < self.offspring_chance => self.propagate(1),
                            _ => vec![],
                        }
                    } else {
                        // use all available moisture even though we take damage
                        section.conditions.moisture = 0;
                        self.damage(1);
                    }
                }
            }
        }
        None
    }

    fn damage(&mut self, damage: i64) {
        self.health = self.health.checked_sub(damage).unwrap_or(0); 
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

        // Optimize for now, since nearby() benchmarks faster than within_range()
        // In the future, establish pseudorandom seed to test that benchmark was accurate.
        let locations = match self.offspring_range {
            1 => self.location.nearby(),
            _ => self.location.within_range(self.offspring_range),
        };
        let pick = rng.gen_range(0..locations.len());
        let location = locations[pick].clone();

        // create new seedling
        let sprout = Plant {
            age: 0,
            flammability_chance: self.flammability_chance,
            health: 1,
            health_max: self.health_max,
            kind: self.kind.clone(),
            // location: Location::new_random(self.location.max),
            location,
            age_max: self.age_max,
            messages: Vec::new(),
            offspring: Vec::new(),
            offspring_chance: self.offspring_chance,
            offspring_range: self.offspring_range,
            on_fire: false,
            requirements: self.requirements.clone(),
            size: 1,
            size_max: self.size_max,
        };
        // change to spawn an extra offspring if health is at max
        let mut offspring: Vec<Plant> = Vec::new();
        for _ in 0..num {
            offspring.push(sprout.clone());
        }
        offspring
    }
}
