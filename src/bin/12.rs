use color_eyre::eyre::Result;
use reformation::Reformation;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Reformation)]
#[reformation(r"{from}-{to}")]
struct Edge {
    from: String,
    to: String,
}

#[derive(Debug)]
struct Room {
    big: bool,
    endpoint: bool,
    neighbors: Vec<String>,
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<String>,
    visited: HashSet<String>,
    double_small: bool,
}

impl Path {
    fn start() -> Path {
        Path {
            path: vec!["start".to_owned()],
            visited: vec!["start".to_owned()].into_iter().collect(),
            double_small: false,
        }
    }

    fn extend(&self, room: &str) -> Path {
        let mut path = self.clone();
        path.path.push(room.to_owned());
        path.visited.insert(room.to_owned());
        path
    }

    fn current_room(&self) -> &str {
        self.path.last().unwrap()
    }

    fn has_visited(&self, room: &str) -> bool {
        self.visited.contains(room)
    }
}

impl Room {
    fn new(name: String) -> Self {
        Self {
            big: name.chars().all(|c| c.is_uppercase()),
            endpoint: false,
            neighbors: Vec::new(),
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;

    let mut map: HashMap<String, Room> = HashMap::new();
    for line in input.split("\n") {
        let edge = Edge::parse(line).unwrap();

        map.entry(edge.from.clone())
            .or_insert_with(|| Room::new(edge.from.clone()))
            .neighbors
            .push(edge.to.clone());

        map.entry(edge.to.clone())
            .or_insert_with(|| Room::new(edge.to))
            .neighbors
            .push(edge.from);
    }
    map.get_mut("start").unwrap().endpoint = true;
    map.get_mut("end").unwrap().endpoint = true;

    // Part One
    let mut valid_paths = Vec::new();
    let mut all_paths = vec![Path::start()];
    while let Some(path) = all_paths.pop() {
        let this_room_name = path.current_room();
        if this_room_name == "end" {
            valid_paths.push(path);
            continue;
        }

        let this_room = map.get(this_room_name).unwrap();
        for next_room_name in &this_room.neighbors {
            let next_room = map.get(next_room_name).unwrap();
            if next_room.big || !path.has_visited(next_room_name) {
                all_paths.push(path.extend(next_room_name));
            }
        }
    }
    dbg!(valid_paths.len());

    // Part Two
    let mut valid_paths = Vec::new();
    let mut all_paths = vec![Path::start()];
    while let Some(path) = all_paths.pop() {
        let this_room_name = path.current_room();
        if this_room_name == "end" {
            valid_paths.push(path);
            continue;
        }

        let this_room = map.get(this_room_name).unwrap();
        for next_room_name in &this_room.neighbors {
            let next_room = map.get(next_room_name).unwrap();
            if next_room.big
                || (!path.double_small && !next_room.endpoint)
                || !path.has_visited(next_room_name)
            {
                let mut new_path = path.extend(next_room_name);
                if !next_room.big && path.has_visited(next_room_name) {
                    new_path.double_small = true;
                }
                all_paths.push(new_path);
            }
        }
    }
    dbg!(valid_paths.len());

    Ok(())
}

#[allow(dead_code)]
const EXAMPLE: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

const INPUT: &'static str = "OU-xt
hq-xt
br-HP
WD-xt
end-br
start-OU
hq-br
MH-hq
MH-start
xt-br
end-WD
hq-start
MH-br
qw-OU
hm-WD
br-WD
OU-hq
xt-MH
qw-MH
WD-qw
end-qw
qw-xt";
