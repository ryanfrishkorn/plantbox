/// Plant entity that has a limited lifespan
#[derive(Debug)]
pub struct Plant {
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
pub struct Rock {
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
