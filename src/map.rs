use crate::board::{Board, Location};

pub struct Map {
    pub board: Board,
    pub matrix: Vec<Vec<char>>,
    pub matrix_scaled: Vec<Vec<char>>,
}

impl Map {
    #[allow(dead_code)]
    fn flip_horizontal(&mut self) {
        self.matrix_scaled.reverse();
    }

    fn flip_vertical(&mut self) {
        let matrix: Vec<Vec<char>> = self.matrix_scaled.clone();
        self.matrix_scaled.clear();
        for mut v in matrix {
            v.reverse();
            self.matrix_scaled.push(v);
        }
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
    pub fn plot_entity(&mut self, location: &Location, c: char) {
        self.matrix[location.x as usize][location.y as usize] = c;
    }

    /// Place character on vector of Location.
    pub fn plot_entities(&mut self, locations: &Vec<Location>, c: char) {
        // plot each type of object
        for l in locations {
            self.matrix[l.x as usize][l.y as usize] = c;
        }
    }

    #[allow(dead_code)]
    fn print_matrix(&self) {
        for row in &self.matrix_scaled {
            for c in row {
                print!("{} ", c);
            }
            print!("\n");
        }
    }

    #[allow(dead_code)]
    fn print_matrix_debug(&self) {
        // Note: All indices printed are respective to the map, not the board or entity locations.

        // matrix[o][] iterate over outer vector
        for (o, outer) in self.matrix_scaled.iter().enumerate() {
            // print single value per row for MAP axis label
            let o_label = (self.matrix_scaled.len() - 1) - o;
            print!("y {:>2} ", o_label);

            // print actual data is which flows along the ascending x-axis
            for c in outer {
                print!("{:<2}", c);
            }
            print!("\n");
        }

        // outer iteration print all MAP indices for easy debugging
        for (o, outer) in self.matrix_scaled.iter().enumerate() {
            if o == 0 {
                print!("   x ");
                // matrix[][i] iterate over inner vector
                for (i, _inner) in outer.iter().enumerate() {
                    print!("{:>2} ", i);
                }
                print!("\n");
            }
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
            // create comparison string to match empty case
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
                // gather all characters that are not default
                // let initials: Vec<char> = s.clone().chars().into_iter().filter_map(|c| { return if c != '.' { Some(c) } else { None } }).collect();
                let initials: Vec<char> = s.clone().chars().into_iter().filter_map(|c| { return if c != '⬛' { Some(c) } else { None } }).collect();
                // print!("{:?} ", initials);

                // look for rock and give it precedence, otherwise print first non-empty char
                let mut i: char = '🪨';
                if initials.contains(&'🪨') == false {
                    i = match initials.first() {
                        Some(c) => *c,
                        _ => 'X',
                    }
                }
                reduced.push(i);
            }
        }

        reduced
    }

    pub fn render(&mut self, scale: i64) {
        // refresh from board reference
        self.matrix_scaled.clear();
        self.matrix_scaled = self.matrix.clone();

        // scaled matrix x-axis
        let mut m: Vec<Vec<char>>;
        m = self.matrix_scaled.clone();
        self.matrix_scaled.clear();
        for row in &m {
            // self.matrix_scaled.clear();
            // reduce x-axis and push to row
            let row_scaled = self.reduce_row(&row, scale);
            self.matrix_scaled.push(row_scaled);
        }

        self.flip_vertical();
        self.rotate(true);
        m.clear();
        m = self.matrix_scaled.clone();
        self.matrix_scaled.clear();
        for row in &m {
            // reduce y-axis (we are rotated)
            let row_scaled = self.reduce_row(row, scale);
            self.matrix_scaled.push(row_scaled);
        }
        self.flip_vertical();

        self.print_matrix_debug();
        // self.print_matrix();
    }

    // fn rotate(&self, matrix: &Vec<Vec<char>>, clockwise: bool) -> Vec<Vec<char>> {
    fn rotate(&mut self, clockwise: bool) {
        let mut matrix_rotated: Vec<Vec<char>> = Vec::new();

        if clockwise {
            let mut matrix_reversed: Vec<Vec<char>> = Vec::new();
            self.matrix_scaled.clone_into(&mut matrix_reversed);
            matrix_reversed.reverse();

            for i in 0..(matrix_reversed[0].len()) {
                matrix_rotated.push(Vec::new());
                for row in &matrix_reversed {
                    matrix_rotated[i].push(row[i]);
                }
            }
        } else {
            let mut matrix_reversed: Vec<Vec<char>> = Vec::new();
            self.matrix_scaled.clone_into(&mut matrix_reversed);
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

        self.matrix_scaled = matrix_rotated;
    }
}
