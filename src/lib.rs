use rand::Rng;

#[derive(Clone, Debug)]
pub struct Board {
    pub matrix: Vec<Vec<BoardSection>>,
    pub size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        // create an empty row
        let mut matrix: Vec<Vec<BoardSection>> = Vec::new();

        for x in 0..size { // x-axis
            let mut row: Vec<BoardSection> = Vec::new();
            for y in 0..size { // y-axis
                let s = BoardSection {
                    conditions: Conditions {
                        light: 0,
                        moisture: 0,
                        oxygen: 0,
                    },
                    location: Location {
                        max: size - 1,
                        x: x as usize,
                        y: y as usize,
                    },
                };
                row.push(s);
            }
            matrix.push(row);
        }

        Board {
            matrix,
            size: size,
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
    pub max: usize,
    pub x: usize,
    pub y: usize,
}

impl Location {
    pub fn set_random(&mut self) {
        self.x = rand::thread_rng().gen_range(0..=self.max);
        self.y = rand::thread_rng().gen_range(0..=self.max);
    }

    pub fn new_random(max: usize) -> Location {
        let mut l = Location::new(max);
        l.set_random();
        l
    }

    pub fn new(max: usize) -> Location {
        let l = Location {
            max: u8::MAX as usize,
            x: 0,
            y: 0,
        };
        l
    }
}

pub struct Map<'a> {
    pub board: &'a Board,
    pub matrix: Vec<Vec<char>>,
    pub matrix_scaled: Vec<Vec<char>>,
}

impl Map<'_> {
    fn flip_horizontal(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut matrix_flipped: Vec<Vec<char>> = Vec::new();
        matrix.clone_into(&mut matrix_flipped);
        matrix_flipped.reverse();

        matrix_flipped
    }

    pub fn new(board: &Board) -> Map {
        // create empty rows
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for y in 0..board.size {
            let mut row: Vec<char> = Vec::new();
            for x in 0..board.size {
                if x == 0 && y == 0 {
                    row.push('0');
                } else {
                    row.push('.');
                }
            }
            if row.len() != board.size {
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

    fn print_matrix(&self, matrix: &Vec<Vec<char>>) {
        for row in matrix {
            for c in row {
                print!("{} ", c);
            }
            print!("\n");
        }
    }

    fn reduce_row(&self, row: &Vec<char>, scale: u8) -> Vec<char> {
        let mut reduced: Vec<char> = Vec::new();

        // check that row len is divisible
        if (row.len() % scale as usize) != 0 {
            panic!("map size and scale factor are not evenly divisible");
        }

        // split and process chunks
        for group in row.chunks(scale as usize) {
            let s: String = group.iter().collect();
            let mut s_compare = String::new();
            for _ in 0..scale {
                s_compare.push('.');
            }

            // println!("comparison - s: {} s_compare: {}", s, s_compare);
            if s == s_compare {
                reduced.push('.'); // empty
            } else {
                reduced.push('X'); // something present
            }
        }

        reduced
    }

    pub fn render(&mut self, scale: u8) {
        // refresh from board reference
        self.matrix_scaled = Vec::new();

        // scaled matrix x-axis
        for row in self.matrix.iter() {
            // reduce x-axis
            let row_scaled = self.reduce_row(row, scale);
            // push to row
            self.matrix_scaled.push(row_scaled);
        }
        // self.print_matrix(&self.matrix_scaled);

        let matrix_rotated = self.rotate(&self.matrix_scaled);
        let mut matrix_scaled = Vec::new();
        for row in matrix_rotated.iter() {
            // reduce y-axis (we are rotated)
            let row_scaled = self.reduce_row(row, scale);
            matrix_scaled.push(row_scaled);
        }
        // FIXME - This is janky and the function should be able to reverse itself.
        let matrix_rotated = self.rotate(&matrix_scaled);
        let matrix_rotated = self.rotate(&matrix_rotated);
        let matrix_rotated = self.rotate(&matrix_rotated);

        // Flip over horizontal axis so location { x: 0, y: 0 } begins at bottom left corner.
        self.matrix_scaled = Map::flip_horizontal(&matrix_rotated);
        self.print_matrix(&self.matrix_scaled);
    }

    fn rotate(&self, matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut matrix_rotated: Vec<Vec<char>> = Vec::new();
        let mut matrix_reversed: Vec<Vec<char>> = Vec::new();
        matrix.clone_into(&mut matrix_reversed);
        matrix_reversed.reverse();
        // println!("matrix_reversed.len(): {}", matrix_reversed.len());

        for i in 0..(matrix_reversed[0].len()) {
            matrix_rotated.push(Vec::new());
            for row in &matrix_reversed {
                // println!("row[{}]: {}", i, row[i]);
                matrix_rotated[i].push(row[i]);
            }
        }

        matrix_rotated
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
