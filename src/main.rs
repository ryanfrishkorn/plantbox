use chrono::Local;
use std::thread::sleep;
use std::time;
use plantbox::*;

fn main() {
    let mut tick: u64 = 0;
    let tick_max: u64 = 1000;
    let map_scale = 8;
    let plant_limit = 20;
    const BOARD_SIZE: usize = u8::MAX as usize;
    let sleep_duration = time::Duration::from_millis(100);

    let mut entities_plants: Vec<Plant> = Vec::new();
    let mut entities_rocks: Vec<Rock> = Vec::new();

    // Create a matrix
    let mut board = Board::new(BOARD_SIZE);

    // Add some plants
    entities_plants.push(Plant::new(PlantKind::Fern, &board));
    entities_plants.push(Plant::new(PlantKind::Tree, &board));

    // Rock object
    let rock = Rock {
        location: Location::new_random(BOARD_SIZE),
    };
    entities_rocks.push(rock);

    loop {
        if tick > tick_max {
            break;
        }
        clear_screen();

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

        // generate new map
        let mut map = Map::new(board.clone());

        let locations: Vec<Location> = entities_rocks.iter().map(|e| e.location.clone()).collect();
        map.plot_entities(&locations, 'R');
        // collect locations of plants that are alive
        for e in entities_plants.iter().filter(|e| e.health > 0) {
            let location = e.location.clone();
            let initial = match e.kind {
                PlantKind::Fern => 'F',
                PlantKind::Tree => 'T',
            };
            // determine initial based on plant kind
            map.plot_entity(location, initial);
        }
        map.render(map_scale);

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
        for _e in &entities_rocks {
            // print!("{} {:?}\n", indent_dyn(1), e);
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

        println!("Plants: {}", entities_plants.len());
        // exit if nothing is alive or we have too many items
        if entities_plants.len() == 0 {
            break;
        }
        if entities_plants.len() > plant_limit {
            // print!("PLANT LIMIT REACHED (clearing)\n");
            // sleep(sleep_duration * 10);
            entities_plants.clear();
            entities_plants.push(Plant::new(PlantKind::Fern, &board));
            entities_plants.push(Plant::new(PlantKind::Tree, &board));
        }

        tick += 1;
        // sleep(sleep_duration);
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}