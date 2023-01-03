use chrono::Local;
use std::thread::sleep;
use std::time;
use plantbox::*;

fn main() {
    let mut tick: u64 = 0;
    let tick_max: u64 = 20;
    let sleep_duration = time::Duration::from_millis(0);

    let mut entities_plants: Vec<Plant> = Vec::new();
    let mut entities_rocks: Vec<Rock> = Vec::new();

    // Create a matrix 256x256
    let board = Board::new();

    // Plant object
    let plant = Plant {
        age: 0,
        health: 10,
        kind: PlantKind::Fern,
        location: Location::new_random(),
        longevity: 12,
        messages: Vec::new(),
    };
    entities_plants.push(plant);

    // Rock object
    let rock = Rock {
        age: 0,
        location: Location::new_random(),
    };
    entities_rocks.push(rock);

    loop {
        sleep(sleep_duration);
        if tick >= tick_max {
            break;
        }
        // establish prefix for log output
        let timestamp = || format!("{} tick: {}", Local::now(), tick);
        let indent = "    ".to_string();
        let indent_dyn = |level: u64| -> String {
            let mut indent_string = "".to_string();
            for _ in 0..level {
                indent_string = indent_string + &indent;
            }
            indent_string
        };

        // print status
        print!("{}\n", timestamp());
        for e in &mut entities_plants {
            for m in &e.messages {
                print!("{} {}\n", indent_dyn(1), m);
            }
            e.messages.clear();
            print!("{} {}", indent_dyn(1), e.summary());
            if !e.alive() {
                print!(" [DEAD]");
            }
            print!("\n");
            // read current conditions for this plant
            print!("{} {:?}\n", indent_dyn(2), board.matrix[e.location.x as usize][e.location.y as usize]);

        }
        for e in &entities_rocks {
            print!("{} {:?}\n", indent_dyn(1), e);
        }

        // propagate environmental conditions to proper locations

        // evolve all entities
        for e in &mut entities_rocks {
            e.evolve();
        }
        for e in &mut entities_plants {
            e.evolve();
        }

        // check all living entities for death
        tick += 1;
    }
}
