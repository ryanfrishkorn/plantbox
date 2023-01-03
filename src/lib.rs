use rand::Rng;

#[derive(Clone, Debug)]
pub struct Board {
    pub matrix: Vec<Vec<BoardSection>>,
    pub size: usize,
}

impl Board {
    pub fn new() -> Board {
        const BOARD_SIZE: usize = 256;
        // create an empty row
        let mut matrix: Vec<Vec<BoardSection>> = Vec::new();

        for x in 0..BOARD_SIZE { // x-axis
            let mut row: Vec<BoardSection> = Vec::new();
            for y in 0..BOARD_SIZE { // y-axis
                let s = BoardSection {
                    conditions: Conditions {
                        light: 0,
                        moisture: 0,
                        oxygen: 0,
                    },
                    x: x as u64,
                    y: y as u64,
                };
                row.push(s);
            }
            matrix.push(row);
        }

        Board {
            matrix,
            size: BOARD_SIZE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoardSection {
    pub conditions: Conditions,
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Debug)]
pub struct Conditions {
    pub light: u64,
    pub moisture: u64,
    pub oxygen: u64,
}

#[derive(Clone, Debug)]
pub enum Effect {
    Light(i64),
    Moisture(i64),
    Oxygen(i64),
}

impl Effect {
    pub fn append_global(&self, board: &mut Board) {
        for row in &mut board.matrix {
            for section in row {
                self.append_to_section(section);
            }
        }
    }

    pub fn apply_global(&self, board: &mut Board) {
        for row in &mut board.matrix {
            for section in row {
                self.apply_to_section(section);
            }
        }
    }

    pub fn append_to_section(&self, section: &mut BoardSection) {
        match self {
            Effect::Light(v) => {
                section.conditions.light = match section.conditions.light.checked_add_signed(*v) {
                    Some(v) => v,
                    None => 0,
                }
            },
            Effect::Moisture(v) => {
                section.conditions.moisture = match section.conditions.moisture.checked_add_signed(*v) {
                    Some(v) => v,
                    None => 0,
                }
            },
            _ => (),
        }
    }

    pub fn apply_to_section(&self, section: &mut BoardSection) {
        match self {
            Effect::Light(v) => {
                section.conditions.light = *v as u64;
            },
            Effect::Moisture(v) => {
                section.conditions.moisture = *v as u64;
            }
            _ => (),
        }
    }
}

/// Location
#[derive(Clone, Debug)]
pub struct Location {
    pub max: u64,
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn set_random(&mut self) {
        self.x = rand::thread_rng().gen_range(0..=self.max as usize);
        self.y = rand::thread_rng().gen_range(0..=self.max as usize);
    }

    pub fn new_random() -> Location {
        let mut l = Location::new();
        l.set_random();
        l
    }

    pub fn new() -> Location {
        let l = Location {
            max: u8::MAX as u64,
            x: 0,
            y: 0,
        };
        l
    }
}

/// Plant entity that has a limited lifespan
#[derive(Clone, Debug)]
pub struct Plant {
    pub age: u64,
    pub health: u64,
    pub kind: PlantKind,
    pub location: Location,
    pub longevity: u64,
    pub messages: Vec<String>,
    pub requirements: Requirements,
}

#[derive(Clone, Debug)]
pub struct Requirements {
    pub light: Effect,
    pub moisture: Effect,
}

impl Plant {
    pub fn summary(&self) -> String {
        format!("Plant {{ kind: {:?} age: {:?}, health: {:?}, longevity: {:?} location: {:?}}}", self.kind, self.age, self.health, self.longevity, self.location)
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
        self.biology(section);
        if self.health == 0 && previous.health != 0 {
            self.messages.push(format!("The {:?} perishes", self.kind));
        }
    }

    fn print_age(&self) {
        println!("plant age: {}", self.age);
    }
}

impl Lifespan for Plant {
    fn alive(&self) -> bool {
        if self.health > 0 {
            return true;
        }
        false
    }

    fn biology(&mut self, section: &mut BoardSection) {
        if self.alive() {
            self.age += 1;
            // death upon exhaustion of lifespan
            if self.age >= self.longevity {
                self.health = 0;
            }

            // Respiration
            match self.requirements.moisture {
                Effect::Moisture(v) => {
                    if section.conditions.moisture >= v as u64 {
                        // consume moisture from section
                        section.conditions.moisture -= v as u64;
                    } else {
                        // take damage
                        self.damage(1);
                    }
                },
                _ => (),
            }
        }
    }

    fn damage(&mut self, damage: u64) {
        self.health = match self.health.checked_sub(damage) {
            Some(v) => v,
            None => 0,
        }
    }
}

/// Rock entity that has a very long lifespan
#[derive(Debug)]
pub struct Rock {
    pub age: u64,
    pub location: Location,
}

impl Rock {
}

impl Evolve for Rock {
    fn evolve(&mut self, section: &mut BoardSection) {
        self.age += 1
    }

    fn print_age(&self) {
        println!("rock age: {}", self.age);
    }
}

/// Father Time wants his incremental payments. All effects that are the result of passing
/// time should be invoked through this trait.
pub trait Evolve {
    fn evolve(&mut self, section: &mut BoardSection);
    fn print_age(&self);
}

pub trait Lifespan {
    fn alive(&self) -> bool;
    fn biology(&mut self, section: &mut BoardSection);
    fn damage(&mut self, damage: u64);
}
