pub mod board;
pub mod evolve;
pub mod map;
pub mod plant;
pub mod rock;

// external
use chrono::Local;
use rand::Rng;
use std::thread::sleep;
use std::time;

// internal
use board::{Board, Effect, Location};
use evolve::{Evolve, Lifespan};
use map::Map;
use plant::{Plant, PlantKind};
use rock::Rock;

fn main() {
    // Map and Board values are interdependent
    const BOARD_SIZE: i64 = 256; // doubling this should result in 4x plant_limit
    const BOARD_MAX: usize = (BOARD_SIZE - 1) as usize;

    let time_start = time::Instant::now();
    let map_size: i64 = 32;
    let map_scale: i64 = BOARD_SIZE / map_size; // this produces 64x64

    // Iteration and sleep
    let sleep_duration = time::Duration::from_millis(0);
    let sleep_duration_burn = time::Duration::from_millis(100);
    let mut tick: u64 = 0;
    let tick_max: u64 = 10000; // 0 for no limit

    let mut entities_plants: Vec<Plant> = Vec::new();
    let mut entities_rocks: Vec<Rock> = Vec::new();
    let ferns_starting = 8;
    let trees_starting = 8;
    let entities_rocks_count = BOARD_SIZE / 8;
    let plant_limit: i64 = (BOARD_SIZE * BOARD_SIZE) - (entities_rocks_count) - ((BOARD_SIZE * BOARD_SIZE) as f64 * 0.1) as i64;
    let mut entities_extinct = false;

    // Create a matrix
    let mut board = Board::new(BOARD_MAX as i64);

    // Add some plants
    for _ in 0..ferns_starting {
        entities_plants.push(Plant::new(PlantKind::Fern, &board));
    }
    for _ in 0..trees_starting {
        entities_plants.push(Plant::new(PlantKind::Tree, &board));
    }

    // Rock objects
    for _ in 0..entities_rocks_count {
        entities_rocks.push(Rock {
            location: Location::new_random(BOARD_MAX as i64),
        });
    }

    loop {
        if tick > tick_max && tick_max != 0 {
            break;
        }
        clear_screen();

        let mut something_burning = false;
        // establish prefix for log output
        let timestamp = || -> String {
            if tick_max == 0 {
                format!("{} tick: {}", Local::now(), tick)
            } else {
                format!("{} tick: {}/{}", Local::now(), tick, tick_max)
            }
        };
        let indent = "    ".to_string();
        let indent_dyn = |level: i64| -> String {
            let mut indent_string = "".to_string();
            for _ in 0..level {
                indent_string = indent_string + &indent;
            }
            indent_string
        };

        // generate new map
        // let mut map = Map::new(board.clone());
        let mut map = Map::new(board.clone());

        // collect locations of plants that are alive
        for e in entities_plants.iter().filter(|e| e.health > 0) {
            // determine initial based on plant kind
            if e.on_fire {
                something_burning = true;
                map.plot_entity(&e.location, '🔥');
            } else {
                map.plot_entity(&e.location, e.kind.icon());
            }
        }

        // Plot rock entities last so they are not overwritten by plants and can take display precedence
        let rock_locations: Vec<Location> =
            entities_rocks.iter().map(|e| e.location.clone()).collect();
        map.plot_entities(&rock_locations, '🪨');
        map.render(map_scale);
        println!("map_scale: {}", map_scale);

        // print status
        println!("{}", timestamp());
        for e in &mut entities_plants {
            for m in &e.messages {
                println!("{} {}", indent_dyn(1), m);
            }
            e.messages.clear();
            if e.alive() {
                // print!("{} {}\n", indent_dyn(1), e.summary());
                // read current conditions for this plant
                // print!("{} {:?}\n", indent_dyn(2), board.matrix[e.location.x as usize][e.location.y as usize]);
            }
        }
        // show first rock
        match &entities_rocks.first() {
            Some(e) => {
                println!("{} {:?}", indent_dyn(1), e);
            }
            None => (),
        }
        /*
        // show all rocks
        for e in &entities_rocks {
            print!("{} {:?}\n", indent_dyn(1), e);
        }
         */

        // set all light values to zero before recalculation cycle
        Effect::Light(0).apply_global(&mut board);
        // light consistently emitted unless modifiers are present from other sources
        let sun = Effect::Light(70);
        sun.apply_global(&mut board);

        // rain is consistent everywhere for now
        let rain = Effect::Moisture(6);
        rain.apply_global(&mut board);

        // evolve all entities
        for e in &mut entities_rocks {
            e.evolve(&mut board.matrix[e.location.x as usize][e.location.y as usize]);
        }

        // let plant_count = entities_plants.len();
        for e in &mut entities_plants {
            // self.x = rand::thread_rng().gen_range(0..=self.max);
            e.evolve(&mut board.matrix[e.location.x as usize][e.location.y as usize]);
        }
        let mut new_plants: Vec<Plant> = Vec::new();
        for e in &mut entities_plants {
            for p in &mut e.offspring {
                // print!("{} NEW {:?}\n", indent_dyn(1), p);
                // sleep(sleep_duration * 10);
                new_plants.push(p.clone());
            }
            e.offspring.clear();
        }
        // push new offspring
        for plant in new_plants {
            entities_plants.push(plant);
        }

        // bring out your dead
        entities_plants.retain(|e| e.alive());

        // show plant statistics
        let fern_count = entities_plants
            .iter()
            .filter(|p| matches!(p.kind, PlantKind::Fern))
            .count();
        // show plant statistics
        let tree_count = entities_plants
            .iter()
            .filter(|p| matches!(p.kind, PlantKind::Tree))
            .count();

        // obtain counts for statistics
        let fern_percent = (fern_count as f32 / entities_plants.len() as f32) * 100.0;
        let tree_percent = (tree_count as f32 / entities_plants.len() as f32) * 100.0;
        println!(
            "{} ferns: {} {:.1}% trees: {} {:.1}% ",
            Local::now(),
            fern_count,
            fern_percent,
            tree_count,
            tree_percent,
        );
        println!(
            "{} plants: {}/{}",
            Local::now(),
            entities_plants.len(),
            plant_limit
        );

        // slash and burn opportunity
        if entities_plants.len() > plant_limit as usize {
            for e in &mut entities_plants {
                let flammable: f64 = rand::thread_rng().gen();
                if flammable < e.flammability_chance {
                    e.on_fire = true;
                }
            }

        }

        if entities_extinct {
            println!("{} Everything is extinct.", Local::now());
            break;
        }

        // check for extinction to break next tick
        if entities_plants.is_empty() {
            entities_extinct = true;
        }

        /* Replant
            print!("{} Everything is extinct. Replanting...\n", Local::now());
            sleep(sleep_duration * 5);
            entities_plants.clear();
            entities_plants.push(Plant::new(PlantKind::Fern, &board));
            entities_plants.push(Plant::new(PlantKind::Tree, &board));
        */

        tick += 1;

        // slow down if things are burning
        if something_burning {
            sleep(sleep_duration_burn);
        } else {
            sleep(sleep_duration);
        }
    }
    let time_stop = time::Instant::now();
    let time_elapsed = time_stop - time_start;
    let ticks_per_second = tick as f32 / time_elapsed.as_secs() as f32;
    println!("program execution time: {:?}", time_elapsed);
    println!("ticks per second: {}", ticks_per_second);
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
