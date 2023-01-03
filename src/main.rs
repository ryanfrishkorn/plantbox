use chrono::Local;
use std::thread::sleep;
use std::time;
use plantbox::*;

fn main() {
    let mut tick: u64 = 0;
    let tick_max: u64 = 20;
    let sleep_duration = time::Duration::from_millis(100);

    let mut entities_plants: Vec<Plant> = Vec::new();
    let mut entities_rocks: Vec<Rock> = Vec::new();

    // Create a matrix 256x256
    let mut board = Board::new();

    // Plant object
    let plant = Plant {
        age: 0,
        health: 10,
        kind: PlantKind::Fern,
        // location: Location::new_random(),
        location: Location {
            max: 256,
            x: 7,
            y: 7,
        },
        longevity: 12,
        messages: Vec::new(),
        requirements: Requirements {
            light: Effect::Light(20),
            moisture: Effect::Moisture(2),
        }
    };
    entities_plants.push(plant);

    // Tree object
    let tree = Plant {
        age: 0,
        health: 18,
        kind: PlantKind::Tree,
        // location: Location::new_random(),
        location: Location {
            max: 256,
            x: 7,
            y: 7,
        },
        longevity: 230,
        messages: Vec::new(),
        requirements: Requirements {
            light: Effect::Light(20),
            moisture: Effect::Moisture(4),
        }
    };
    entities_plants.push(tree);

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
            if e.alive() {
                print!("{} {}\n", indent_dyn(1), e.summary());
                // read current conditions for this plant
                print!("{} {:?}\n", indent_dyn(2), board.matrix[e.location.x as usize][e.location.y as usize]);
            }
        }
        for e in &entities_rocks {
            print!("{} {:?}\n", indent_dyn(1), e);
        }

        // set all light values to zero before recalculation cycle
        Effect::Light(0).apply_global(&mut board);
        // light consistently emitted unless modifiers are present from other sources
        let sun = Effect::Light(70);
        sun.apply_global(&mut board);

        // rain is consistent everywhere for now
        let rain = Effect::Moisture(5);
        rain.apply_global(&mut board);

        // evolve all entities
        for e in &mut entities_rocks {
            e.evolve(&mut board.matrix[e.location.x][e.location.y]);
        }
        for e in &mut entities_plants {
            e.evolve(&mut board.matrix[e.location.x][e.location.y]);
        }

        // check all living entities for death
        tick += 1;
    }
}
