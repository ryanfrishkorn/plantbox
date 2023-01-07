use rand::{Rng};

#[derive(Clone, Debug)]
pub struct Board {
    pub matrix: Vec<Vec<BoardSection>>,
    pub size: i64,
}

impl Board {
    pub fn new(size: i64) -> Board {
        // create an empty row
        let mut matrix: Vec<Vec<BoardSection>> = Vec::new();

        for x in 0..=size { // x-axis
            let mut row: Vec<BoardSection> = Vec::new();
            for y in 0..=size { // y-axis
                let s = BoardSection {
                    conditions: Conditions {
                        light: 0,
                        moisture: 0,
                        oxygen: 0,
                    },
                    location: Location {
                        max: size,
                        x: x as i64,
                        y: y as i64,
                    },
                };
                row.push(s);
            }
            matrix.push(row);
        }

        Board {
            matrix,
            size,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoardSection {
    pub conditions: Conditions,
    pub location: Location,
}

#[derive(Clone, Debug)]
pub struct Conditions {
    pub light: i64,
    pub moisture: i64,
    pub oxygen: i64,
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
            Effect::Light(v) => section.conditions.light += *v,
            Effect::Moisture(v) => section.conditions.moisture += *v,
            _ => (),
        }
    }

    pub fn apply_to_section(&self, section: &mut BoardSection) {
        match self {
            Effect::Light(v) => {
                section.conditions.light = *v as i64;
            },
            Effect::Moisture(v) => {
                section.conditions.moisture = *v as i64;
            }
            _ => (),
        }
    }
}

/// Location
#[derive(Clone, Debug)]
pub struct Location {
    pub max: i64,
    pub x: i64,
    pub y: i64,
}

impl Location {
    // return a vector of all surrounding locations
    pub fn nearby(&self) -> Vec<Location> {
        // theoretical data
        // g - h - i    (0,2) - (1,2) - (2,2)
        // d - x - f    (0,1) - (1,1) - (2,1)
        // a - b - c    (0,0) - (1,0) - (2,0)

        // corner case
        // let a = (0, 0); // [3] -> d (0,1) x (1,1) b (1,0)
        // edge case
        // let h = (1, 2); // [5] -> d (0,1) d (0,1) x (1,1) f (2,1) i (2,2)
        // optimal case
        // let x = (1, 1); // [8] -> a (0,0) d (0,1) g (0,2) h (1,2) i (2,2) f (2,1) c (2,0) b (1,0)

        let mut locations: Vec<Location> = Vec::new();

        // subtract both to start at lower corner
        let mut loc: Location = self.clone();
        // a (0,0)
        loc.x -= 1;
        loc.y -= 1;
        locations.push(loc.clone());
        // b (1,0)
        loc.x += 1; // b (1,0)
        locations.push(loc.clone());
        // c (2,0)
        loc.x += 1;
        locations.push(loc.clone());
        // f (2,1)
        loc.y += 1;
        locations.push(loc.clone());
        // i (2,2)
        loc.y += 1;
        locations.push(loc.clone());
        // h (1,2)
        loc.x -= 1;
        locations.push(loc.clone());
        // g (0, 2)
        loc.x -= 1;
        locations.push(loc.clone());
        // d (0, 1)
        loc.y -= 1;
        locations.push(loc.clone());

        // filter out all locations with negative coordinates
        locations = locations.into_iter().filter(|c| { !c.x.is_negative() && !c.y.is_negative() }).collect();
        // filter out all locations with coordinates beyond maximum
        locations = locations.into_iter().filter(|c| { c.x <= self.max && c.y <= self.max }).collect();
        locations
    }

    pub fn set_random(&mut self) {
        self.x = rand::thread_rng().gen_range(0..=self.max);
        self.y = rand::thread_rng().gen_range(0..=self.max);
    }

    pub fn new_random(max: i64) -> Location {
        let mut l = Location::new(max);
        l.set_random();
        l
    }

    pub fn new(max: i64) -> Location {
        let l = Location {
            max,
            x: 0,
            y: 0,
        };
        l
    }
}
