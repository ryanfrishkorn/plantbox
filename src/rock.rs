use crate::evolve::Evolve;
use crate::board::{BoardSection, Location};

/// Rock entity that has a very long lifespan
#[derive(Clone, Debug)]
pub struct Rock {
    pub location: Location,
}

impl Rock {
}

impl Evolve for Rock {
    fn evolve(&mut self, _section: &mut BoardSection) {
    }
}
