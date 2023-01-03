use rand::Rng;

#[derive(Debug)]
pub struct Board {
    pub matrix: Vec<Vec<BoardSection>>,
}

impl Board {
    pub fn new() -> Board {
        const BOARD_SIZE: usize = 256;
        // create an empty row
        let mut matrix: Vec<Vec<BoardSection>> = Vec::new();
        let mut row: Vec<BoardSection> = Vec::new();

        for _ in 0..BOARD_SIZE {
            let s = BoardSection {
                conditions: Conditions {
                    moisture: 0,
                    oxygen: 0,
                },
            };
            row.push(s);
        }
        for _ in 0..BOARD_SIZE {
            matrix.push(row.clone());
        }

        Board {
            matrix,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoardSection {
    pub conditions: Conditions,
}

#[derive(Clone, Debug)]
pub struct Conditions {
    pub moisture: u64,
    pub oxygen: u64,
}

/// Location
#[derive(Clone, Debug)]
pub struct Location {
    pub max: u64,
    pub x: u64,
    pub y: u64,
}

impl Location {
    pub fn set_random(&mut self) {
        self.x = rand::thread_rng().gen_range(0..=self.max);
        self.y = rand::thread_rng().gen_range(0..=self.max);
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
    pub health: u16,
    pub kind: PlantKind,
    pub location: Location,
    pub longevity: u64,
    pub messages: Vec<String>,
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
    fn evolve(&mut self) {
        // Save current state for comparison after evolution
        let previous = self.clone();
        self.biology();
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

    fn biology(&mut self) {
        if self.alive() {
            self.age += 1;
            // death upon exhaustion of lifespan
            if self.age >= self.longevity {
                self.health = 0;
            }
            // respiration
            self.breathe();
        }
    }

    fn breathe(&self) {
    }
}

/// Rock entity that has a very long lifespan
#[derive(Debug)]
pub struct Rock {
    pub age: u64,
    pub location: Location,
}

impl Rock {
    fn biology(&mut self) {
        self.age += 1;
    }
}

impl Evolve for Rock {
    fn evolve(&mut self) {
        self.biology();
    }

    fn print_age(&self) {
        println!("rock age: {}", self.age);
    }
}

/// Father Time wants his incremental payments. All effects that are the result of passing
/// time should be invoked through this trait.
pub trait Evolve {
    fn evolve(&mut self);
    fn print_age(&self);
}

pub trait Lifespan {
    fn alive(&self) -> bool;
    fn biology(&mut self);
    fn breathe(&self);
}
