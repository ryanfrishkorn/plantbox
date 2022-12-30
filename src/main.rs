use std::ops::DerefMut;
use std::thread::sleep;
use std::time;

/// Father Time wants his incremental payments.
pub trait Evolve {
    fn evolve(&mut self);
    fn print_age(&self);
}

#[derive(Debug)]
enum Entity {
    Plant(Plant),
    Rock(Rock),
}

/// Plant entity that has a limited lifespan
#[derive(Debug)]
struct Plant {
    pub age: u64,
    pub health: u16,
}

impl Plant {
    fn age_increment(&mut self) {
        self.age += 1;
    }
}

impl Evolve for Plant {
    fn evolve(&mut self) {
        self.age_increment();
    }

    fn print_age(&self) {
        println!("plant age: {}", self.age);
    }
}

/// Rock entity that has a very long lifespan
#[derive(Debug)]
struct Rock {
    pub age: u64,
    pub health: u16,
}

impl Rock {
    fn age_increment(&mut self) {
        self.age += 1;
    }
}

impl Evolve for Rock {
    fn evolve(&mut self) {
        self.age_increment();
    }

    fn print_age(&self) {
        println!("rock age: {}", self.age);
    }
}


fn main() {
    let mut tick: u64 = 0;
    let tick_max: u64 = 100;
    let sleep_duration = time::Duration::from_millis(10);

    // let mut entities: Vec<Box<dyn Evolve>> = Vec::new();
    let mut entities: Vec<Box<dyn Evolve>> = Vec::new();

    // Rock object
    let rock = Rock {
        age: 0,
        health: 1024,
    };
    entities.push(Box::new(rock));

    // Plant object
    let plant = Plant {
        age: 0,
        health: 10,
    };
    entities.push(Box::new(plant));

    loop {
        sleep(sleep_duration);
        if tick >= tick_max {
            break;
        }
        // evolve all entities
        for e in &mut entities {
            e.evolve();
            e.print_age();
        }

        tick += 1;
    }
}
