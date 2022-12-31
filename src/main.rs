use chrono::Local;
use std::thread::sleep;
use std::time;

/// Plant entity that has a limited lifespan
#[derive(Debug)]
struct Plant {
    pub age: u64,
    pub health: u16,
    pub longevity: u64,
}

impl Plant {
}

impl Evolve for Plant {
    fn evolve(&mut self) {
        self.biology();
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
        self.age += 1;
        // death upon exhaustion of lifespan
        if self.age >= self.longevity {
            self.health = 0;
        }
        // respiration
        self.breathe();
    }

    fn breathe(&self) {
    }
}

/// Rock entity that has a very long lifespan
#[derive(Debug)]
struct Rock {
    pub age: u64,
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

fn main() {
    let mut tick: u64 = 0;
    let tick_max: u64 = 20;
    let sleep_duration = time::Duration::from_millis(10);

    let mut entities_plants: Vec<Plant> = Vec::new();
    let mut entities_rocks: Vec<Rock> = Vec::new();

    // Plant object
    let plant = Plant {
        age: 0,
        health: 10,
        longevity: 12,
    };
    entities_plants.push(plant);

    // Rock object
    let rock = Rock {
        age: 0,
    };
    entities_rocks.push(rock);

    // let mut entities_lifespan: Vec<Box<dyn Lifespan>> = Vec::new();

    loop {
        sleep(sleep_duration);
        if tick >= tick_max {
            break;
        }
        // establish prefix for log output
        let timestamp = || format!("{} tick: {}", Local::now(), tick);
        let indent = "    ".to_string();
        let indent_dyn = |level: u64| -> String {
            let mut x = level.clone();
            let mut indent_string = "".to_string();
            while x > 0 {
                indent_string = indent_string + &indent;
                x -= 1;
            }
            indent_string
        };

        // print status
        print!("{}\n", timestamp());
        for e in &entities_plants {
            if e.alive() {
                print!("{} {:?}\n", indent_dyn(1), e);
            }
        }
        for e in &entities_rocks {
            print!("{} {:?}\n", indent_dyn(1), e);
        }

        // evolve all entities
        for e in &mut entities_rocks {
            e.evolve();
        }
        for e in &mut entities_plants {
            e.evolve();
        }

        // check all living entities for death
        tick += 1;
    }
}
