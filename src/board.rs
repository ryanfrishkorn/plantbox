use rand::Rng;

#[derive(Clone, Debug)]
pub struct Board {
    pub matrix: Vec<Vec<BoardSection>>,
    pub size: i64,
}

impl Board {
    pub fn new(size: i64) -> Board {
        // create an empty row
        let mut matrix: Vec<Vec<BoardSection>> = Vec::new();

        for x in 0..=size {
            // x-axis
            let mut row: Vec<BoardSection> = Vec::new();
            for y in 0..=size {
                // y-axis
                let s = BoardSection {
                    conditions: Conditions {
                        light: 0,
                        moisture: 0,
                        oxygen: 0,
                    },
                    location: Location {
                        max: size,
                        x,
                        y,
                    },
                };
                row.push(s);
            }
            matrix.push(row);
        }

        Board { matrix, size }
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
                section.conditions.light = *v;
            }
            Effect::Moisture(v) => {
                section.conditions.moisture = *v;
            }
            _ => (),
        }
    }
}

/// Location
#[derive(Clone, Debug, PartialEq)]
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
        locations.push(loc);

        // filter out all locations with negative coordinates
        locations.retain(|c| !c.x.is_negative() && !c.y.is_negative());
        // filter out all locations with coordinates beyond maximum
        locations.retain(|c| c.x <= self.max && c.y <= self.max);
        locations
    }

    /// Return a vector of possible destinations within a specified range.
    pub fn within_range(&self, range: i64) -> Vec<Location> {
        let mut locations: Vec<Location> = Vec::new();

        // This is a closure to return a vector of possible values. These values are
        // determined by capturing the "range" parameter of the containing function.
        let min_max = |l: i64| -> Vec<i64> {
            // this is the minimum possible including zero
            let min = match l - range {
                x if x.is_negative() => 0,
                x => x,
            };
            // this is the maximum possible value limited by board size
            let max = match l + range {
                x if x > self.max => self.max,
                x => x,
            };
            (min..=max).collect()
        };

        for x in min_max(self.x) {
            for y in min_max(self.y) {
                // exclude current location from results
                if self.x == x && self.y == y {
                    continue;
                }
                locations.push(Location {
                    max: self.max,
                    x,
                    y,
                });
            }
        }

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
        Location { max, x: 0, y: 0 }
    }
}

mod tests {
    #[test]
    fn benchmark_movement_calc() {
        use crate::Location;

        let l = Location::new_random(255);

        let start = std::time::Instant::now();
        for _ in 0..1 {
            let _a = l.within_range(1);
        }
        let stop = std::time::Instant::now();
        println!("within_range: {:?}", stop - start);

        let start = std::time::Instant::now();
        for _ in 0..1 {
            let _a = l.nearby();
        }
        let stop = std::time::Instant::now();
        println!("nearby: {:?}", stop - start);
    }

    #[test]
    fn location_nearby() {
        use crate::Location;
        let max = 255;

        // Location 0, 0
        let mut l = Location { max, x: 0, y: 0 };

        let mut expected: Vec<Location> = Vec::new();
        expected.push(Location { max, x: 0, y: 1 });
        expected.push(Location { max, x: 1, y: 1 });
        expected.push(Location { max, x: 1, y: 0 });
        let result = l.nearby();
        assert_eq!(result.len(), expected.len());
        for location in result {
            assert!(expected.contains(&location));
        }

        // Location 1, 1
        (l.x, l.y) = (1, 1);
        expected.clear();

        let mut expected: Vec<Location> = Vec::new();
        expected.push(Location { max, x: 0, y: 0 });
        expected.push(Location { max, x: 0, y: 1 });
        expected.push(Location { max, x: 0, y: 2 });

        expected.push(Location { max, x: 1, y: 0 });
        // do not include self
        // expected.push(Location { max, x: 1, y: 1 });
        expected.push(Location { max, x: 1, y: 2 });

        expected.push(Location { max, x: 2, y: 0 });
        expected.push(Location { max, x: 2, y: 1 });
        expected.push(Location { max, x: 2, y: 2 });

        let result = l.nearby();
        // println!("result: {:?}", result);
        // println!("expect: {:?}", expected);
        assert_eq!(result.len(), expected.len());
        for location in result {
            assert!(expected.contains(&location));
        }
    }

    #[test]
    #[rustfmt::skip] // prevent expansion of simple Location struct literals
    fn location_within_range() {
        use crate::Location;

        let max = 255;
        let mut location = Location { max, x: 0, y: 0 };
        let mut expected: Vec<Location> = Vec::new();
        let mut results: Vec<Location>;

        let check_results = |results: &Vec<Location>, expected: &Vec<Location>| {
            // ensure that number of locations is the same
            assert_eq!(results.len(), expected.len());

            for l in results {
                assert!(expected.contains(&l));
            }
            // reverse case in case of duplicate values
            for l in expected {
                assert!(results.contains(&l));
            }
            true
        };

        // 0, 0 (lower-left corner)
        (location.x, location.y) = (0, 0);
        expected.clear();
        expected.push(Location { max, x: 0, y: 1 });
        expected.push(Location { max, x: 1, y: 1 });
        expected.push(Location { max, x: 1, y: 0 });

        results = location.within_range(1);
        check_results(&results, &expected);

        // 255, 0 (lower-right corner)
        (location.x, location.y) = (255, 0);
        expected.clear();
        expected.push(Location { max, x: 254, y: 0 });
        expected.push(Location { max, x: 255, y: 1 });
        expected.push(Location { max, x: 254, y: 1 });

        results = location.within_range(1);
        check_results(&results, &expected);

       // 0, 255 (upper-left corner)
        (location.x, location.y) = (0, 255);
        expected.clear();
        expected.push(Location { max, x: 0, y: 254 });
        expected.push(Location { max, x: 1, y: 254 });
        expected.push(Location { max, x: 1, y: 255 });

        results = location.within_range(1);
        check_results(&results, &expected);
        
        // 255, 255 (upper-right corner)
        (location.x, location.y) = (255, 255);
        expected.clear();
        expected.push(Location { max, x: 255, y: 254 });
        expected.push(Location { max, x: 254, y: 254 });
        expected.push(Location { max, x: 254, y: 255 });

        results = location.within_range(1);
        check_results(&results, &expected);

        // 1, 1
        (location.x, location.y) = (1, 1);
        expected.clear();
        expected.push(Location { max, x: 0, y: 0 });
        expected.push(Location { max, x: 0, y: 1 });
        expected.push(Location { max, x: 0, y: 2 });

        expected.push(Location { max, x: 1, y: 0 });
        expected.push(Location { max, x: 1, y: 2 });

        expected.push(Location { max, x: 2, y: 0 });
        expected.push(Location { max, x: 2, y: 1 });
        expected.push(Location { max, x: 2, y: 2 });

        results = location.within_range(1);
        check_results(&results, &expected);
    }
}
