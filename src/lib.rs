use rand::{Rng, rngs::StdRng, SeedableRng};

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

pub struct Map {
    pub board: Board,
    pub matrix: Vec<Vec<char>>,
    pub matrix_scaled: Vec<Vec<char>>,
}

impl Map {
    fn flip_horizontal(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut matrix_flipped: Vec<Vec<char>> = Vec::new();
        matrix.clone_into(&mut matrix_flipped);
        matrix_flipped.reverse();

        matrix_flipped
    }

    pub fn new(board: Board) -> Map {
        // create empty rows
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for _y in 0..=board.size {
            let mut row: Vec<char> = Vec::new();
            for _x in 0..=board.size {
                row.push('⬛');
            }
            if row.len() as i64 != board.size + 1 {
                panic!("row.len(): {}", row.len());
            }
            matrix.push(row);
        }

        Map {
            board,
            matrix: matrix.clone(),
            matrix_scaled: matrix,
        }
    }

    /// Place character on specified Location.
    pub fn plot_entity(&mut self, location: Location, c: char) {
        self.matrix[location.x as usize][location.y as usize] = c;
    }

    /// Place character on vector of Location.
    pub fn plot_entities(&mut self, locations: &Vec<Location>, c: char) {
        // plot each type of object
        for l in locations {
            self.matrix[l.x as usize][l.y as usize] = c;
        }
    }

    fn print_matrix(&self, matrix: &Vec<Vec<char>>) {
        for row in matrix {
            for c in row {
                print!("{} ", c);
            }
            print!("\n");
        }
    }

    fn reduce_row(&self, row: &Vec<char>, scale: i64) -> Vec<char> {
        let mut reduced: Vec<char> = Vec::new();

        // check that row len is divisible
        if (row.len() % scale as usize) != 0 {
            panic!("map size and scale factor are not evenly divisible - row.len(): {}", row.len());
        }

        // split and process chunks
        for group in row.chunks(scale as usize) {
            let s: String = group.iter().collect();
            let mut s_compare = String::new();
            for _ in 0..scale {
                // s_compare.push('.');
                s_compare.push('⬛');
            }

            // println!("comparison - s: {} s_compare: {}", s, s_compare);
            if s == s_compare {
                // reduced.push('.'); // empty
                reduced.push('⬛');
            } else {
                // detect first character that is not '.'
                // let initials: Vec<char> = s.clone().chars().into_iter().filter_map(|c| { return if c != '.' { Some(c) } else { None } }).collect();
                let initials: Vec<char> = s.clone().chars().into_iter().filter_map(|c| { return if c != '⬛' { Some(c) } else { None } }).collect();
                let i = match initials.first() {
                    Some(c) => *c,
                    _ => 'X',
                };
                reduced.push(i);
            }
        }

        reduced
    }

    pub fn render(&mut self, scale: i64) {
        // refresh from board reference
        self.matrix_scaled = Vec::new();

        // scaled matrix x-axis
        for row in self.matrix.iter() {
            // reduce x-axis and push to row
            let row_scaled = self.reduce_row(row, scale);
            self.matrix_scaled.push(row_scaled);
        }

        let matrix_rotated = self.rotate(&self.matrix_scaled, true); // clockwise
        let mut matrix_scaled = Vec::new();
        for row in matrix_rotated.iter() {
            // reduce y-axis (we are rotated)
            let row_scaled = self.reduce_row(row, scale);
            matrix_scaled.push(row_scaled);
        }
        let matrix_rotated = self.rotate(&matrix_scaled, false); // counter-clockwise

        // Flip over horizontal axis so location { x: 0, y: 0 } begins at bottom left corner.
        self.matrix_scaled = Map::flip_horizontal(&matrix_rotated);
        self.print_matrix(&self.matrix_scaled);
    }

    fn rotate(&self, matrix: &Vec<Vec<char>>, clockwise: bool) -> Vec<Vec<char>> {
        let mut matrix_rotated: Vec<Vec<char>> = Vec::new();

        if clockwise {
            let mut matrix_reversed: Vec<Vec<char>> = Vec::new();
            matrix.clone_into(&mut matrix_reversed);
            matrix_reversed.reverse();

            for i in 0..(matrix_reversed[0].len()) {
                matrix_rotated.push(Vec::new());
                for row in &matrix_reversed {
                    matrix_rotated[i].push(row[i]);
                }
            }
        } else {
            let mut matrix_reversed: Vec<Vec<char>> = Vec::new();
            matrix.clone_into(&mut matrix_reversed);
            for row in &mut matrix_reversed {
                row.reverse();
            }

            for i in 0..matrix_reversed[0].len() {
                matrix_rotated.push(Vec::new());
                for row in &matrix_reversed {
                    matrix_rotated[i].push(row[i]);
                }
            }
        }

        matrix_rotated
    }
}

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
                        // take damage
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
