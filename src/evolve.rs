use crate::board::{BoardSection};
use crate::plant::{Plant};

/// Father Time wants his incremental payments. All effects that are the result of passing
/// time should be invoked through this trait.
pub trait Evolve {
    fn evolve(&mut self, section: &mut BoardSection);
}

pub trait Lifespan {
    fn alive(&self) -> bool;
    fn biology(&mut self, section: &mut BoardSection) -> Option<Vec<Plant>>;
    fn damage(&mut self, damage: i64);
    fn grow(&mut self);
    fn propagate(&mut self, num: i64) -> Vec<Plant>;
}
