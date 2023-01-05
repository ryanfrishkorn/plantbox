use chrono::Local;
use std::thread::sleep;
use std::time;
use plantbox::*;

fn main() {
    const BOARD_SIZE: i64 = 256; // doubling this should result in 4x plant_limit
    const BOARD_MAX: usize = (BOARD_SIZE - 1) as usize;
    const BOARD_UNIT: i64 = 8;

    let map_scale: i64 = BOARD_SIZE / (BOARD_UNIT * 2 * 2);
    // let map_scale: i64 = 1;
    println!("MAP_SCALE: {}", map_scale);
    let plant_limit_base: i64 = 62; // derived from theoretical board of 8
    // let plant_limit_base: i64 = 250; // derived from theoretical board of 16
    let plant_limit: i64 = ((BOARD_SIZE / BOARD_UNIT) * (BOARD_SIZE / BOARD_UNIT)) * plant_limit_base;

    let sleep_duration = time::Duration::from_millis(0);
    let mut tick: u64 = 0;
    let tick_max: u64 = 0; // 0 for no limit

    let mut entities_plants: Vec<Plant> = Vec::new();
    let mut entities_rocks: Vec<Rock> = Vec::new();

    // Create a matrix
    let mut board = Board::new(BOARD_MAX as i64);

    // Add some plants
    entities_plants.push(Plant::new(PlantKind::Fern, &board));
    entities_plants.push(Plant::new(PlantKind::Fern, &board));
    entities_plants.push(Plant::new(PlantKind::Fern, &board));
    entities_plants.push(Plant::new(PlantKind::Fern, &board));
    entities_plants.push(Plant::new(PlantKind::Tree, &board));
    entities_plants.push(Plant::new(PlantKind::Tree, &board));

    // Rock object
    entities_rocks.push(Rock { location: Location::new_random(BOARD_MAX as i64) });
    entities_rocks.push(Rock { location: Location::new_random(BOARD_MAX as i64) });
    entities_rocks.push(Rock { location: Location::new_random(BOARD_MAX as i64) });
    entities_rocks.push(Rock { location: Location::new_random(BOARD_MAX as i64) });

    loop {
        if tick > tick_max && tick_max != 0 {
            break;
        }
        clear_screen();

        // establish prefix for log output
        let timestamp = || format!("{} tick: {}", Local::now(), tick);
        let indent = "    ".to_string();
        let indent_dyn = |level: i64| -> String {
            let mut indent_string = "".to_string();
            for _ in 0..level {
                indent_string = indent_string + &indent;
            }
            indent_string
        };

        // generate new map
        let mut map = Map::new(board.clone());

        let locations: Vec<Location> = entities_rocks.iter().map(|e| e.location.clone()).collect();
        // map.plot_entities(&locations, 'R');
        // map.plot_entities(&locations, '🗿');
        map.plot_entities(&locations, '🪨');
        // collect locations of plants that are alive
        for e in entities_plants.iter().filter(|e| e.health > 0) {
            let location = e.location.clone();
            let initial = match e.kind {
                PlantKind::Fern => '🌿',
                PlantKind::Tree => '🌲',
            };
            // determine initial based on plant kind
            map.plot_entity(location, initial);
        }
        map.render(map_scale);
        println!("map_scale: {}", map_scale);

        // print status
        print!("{}\n", timestamp());
        for e in &mut entities_plants {
            for m in &e.messages {
                print!("{} {}\n", indent_dyn(1), m);
            }
            e.messages.clear();
            if e.alive() {
                // print!("{} {}\n", indent_dyn(1), e.summary());
                // read current conditions for this plant
                // print!("{} {:?}\n", indent_dyn(2), board.matrix[e.location.x as usize][e.location.y as usize]);
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
            e.evolve(&mut board.matrix[e.location.x as usize][e.location.y as usize]);
        }

        for e in &mut entities_plants {
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

        print!("{} plants: {}/{}\n", Local::now(), entities_plants.len(), plant_limit);

        // clear and replant if plant limit is reached
        if entities_plants.len() > plant_limit as usize {
            break;
        }
        // pause, clear, and replant if everything is extinct
        if entities_plants.len() == 0 {
            print!("{} Everything is extinct. Replanting...\n", Local::now());
            sleep(sleep_duration * 5);
            entities_plants.clear();
            entities_plants.push(Plant::new(PlantKind::Fern, &board));
            entities_plants.push(Plant::new(PlantKind::Tree, &board));
        }

        tick += 1;
        sleep(sleep_duration);
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}