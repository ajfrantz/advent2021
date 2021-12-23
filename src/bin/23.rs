use color_eyre::eyre::Result;

use pathfinding::prelude::{dijkstra, dijkstra_all};
use std::collections::HashMap;

// If the 7 hallway spaces not in front of doors can be in ~5 states (empty/a/b/c/d) and the rooms
// can be in ~3 states (empty/correct/incorrect) then we have something like ~512M states?

// Since nobody will ever stop on the hallway rooms in front of the entrance, we don't need to
// model them. That leaves us with 15 places on the grid:
//
// [0, 6] are the hallway rooms *not* in front of a door, left to right
// 7+8 are Amber rooms
// 9+10 are Bronze rooms
// 11+12 are Copper rooms
// 13+14 are Desert rooms
//
// Part two addes two more 'home' rooms.
//
//  15+16 are also Amber rooms in part two
//  17+18 are also Bronze rooms in part two
//  19+20 are also Copper rooms in part two
//  21+22 are also Desert rooms in part two

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber = 1,
    Bronze = 2,
    Copper = 3,
    Desert = 4,
}

impl Amphipod {
    fn mask(self) -> u8 {
        1 << (self as u8)
    }

    fn move_multiplier(self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }

    fn home(self) -> [usize; 4] {
        match self {
            Amphipod::Amber => [7, 8, 15, 16],
            Amphipod::Bronze => [9, 10, 17, 18],
            Amphipod::Copper => [11, 12, 19, 20],
            Amphipod::Desert => [13, 14, 21, 22],
        }
    }

    fn allowed_at(self, position: usize) -> bool {
        (ALLOWED[position] & self.mask()) != 0
    }
}

const A: Amphipod = Amphipod::Amber;
const B: Amphipod = Amphipod::Bronze;
const C: Amphipod = Amphipod::Copper;
const D: Amphipod = Amphipod::Desert;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Burrow {
    diagram: [Option<Amphipod>; 15 + 8],
    map_size: usize,
}

// For each place on the diagram, the list of places it's connected to and an associated
// multiplier. (The multiplier captures that 'skipping' a door costs 2 moves.)
const ADJACENCY: &'static [&'static [(usize, usize)]] = &[
    &[(1, 1)],
    &[(0, 1), (2, 2), (7, 2)],
    &[(1, 2), (7, 2), (9, 2), (3, 2)],
    &[(2, 2), (9, 2), (11, 2), (4, 2)],
    &[(3, 2), (11, 2), (13, 2), (5, 2)],
    &[(4, 2), (13, 2), (6, 1)],
    &[(5, 1)],
    &[(1, 2), (2, 2), (8, 1)],
    &[(7, 1), (15, 1)],
    &[(2, 2), (3, 2), (10, 1)],
    &[(9, 1), (17, 1)],
    &[(3, 2), (4, 2), (12, 1)],
    &[(11, 1), (19, 1)],
    &[(4, 2), (5, 2), (14, 1)],
    &[(13, 1), (21, 1)],
    // Added in part two.
    &[(8, 1), (16, 1)],
    &[(15, 1)],
    &[(10, 1), (18, 1)],
    &[(17, 1)],
    &[(12, 1), (20, 1)],
    &[(19, 1)],
    &[(14, 1), (22, 1)],
    &[(21, 1)],
];

const ALLOWED: [u8; 15 + 8] = [
    0b1111_0, // 0
    0b1111_0, // 1
    0b1111_0, // 2
    0b1111_0, // 3
    0b1111_0, // 4
    0b1111_0, // 5
    0b1111_0, // 6
    0b0001_0, // 7
    0b0001_0, // 8
    0b0010_0, // 9
    0b0010_0, // 10
    0b0100_0, // 11
    0b0100_0, // 12
    0b1000_0, // 13
    0b1000_0, // 14
    // Added in part two.
    0b0001_0, // 15
    0b0001_0, // 16
    0b0010_0, // 17
    0b0010_0, // 18
    0b0100_0, // 19
    0b0100_0, // 20
    0b1000_0, // 21
    0b1000_0, // 22
];

impl Burrow {
    fn for_part_one(setup: [Amphipod; 8]) -> Burrow {
        let mut diagram = [None; 15 + 8];
        diagram[7] = Some(setup[0]);
        diagram[8] = Some(setup[4]);
        diagram[9] = Some(setup[1]);
        diagram[10] = Some(setup[5]);
        diagram[11] = Some(setup[2]);
        diagram[12] = Some(setup[6]);
        diagram[13] = Some(setup[3]);
        diagram[14] = Some(setup[7]);

        Burrow {
            diagram,
            map_size: 15,
        }
    }

    fn for_part_two(setup: [Amphipod; 16]) -> Burrow {
        let mut diagram = [None; 15 + 8];
        diagram[7] = Some(setup[0]);
        diagram[8] = Some(setup[4]);
        diagram[15] = Some(setup[8]);
        diagram[16] = Some(setup[12]);

        diagram[9] = Some(setup[1]);
        diagram[10] = Some(setup[5]);
        diagram[17] = Some(setup[9]);
        diagram[18] = Some(setup[13]);

        diagram[11] = Some(setup[2]);
        diagram[12] = Some(setup[6]);
        diagram[19] = Some(setup[10]);
        diagram[20] = Some(setup[14]);

        diagram[13] = Some(setup[3]);
        diagram[14] = Some(setup[7]);
        diagram[21] = Some(setup[11]);
        diagram[22] = Some(setup[15]);

        Burrow {
            diagram,
            map_size: 15 + 8,
        }
    }

    fn moved(&self, from: usize, to: usize) -> Burrow {
        let mut result = *self;
        assert!(from < self.map_size);
        assert!(to < self.map_size);
        assert!(result.diagram[from].is_some());
        assert!(result.diagram[to].is_none());
        result.diagram[to] = result.diagram[from].take();

        result
    }

    fn reachable(&self) -> Vec<(Burrow, usize)> {
        let mut successors = Vec::new();
        for (pos, amphipod) in self.diagram.iter().enumerate() {
            if let Some(amphipod) = amphipod {
                // Figure out which rooms we could even get to if we wanted to.
                let possible_targets: HashMap<usize, (usize, usize)> = dijkstra_all(&pos, |&p| {
                    ADJACENCY[p]
                        .iter()
                        .cloned()
                        .filter(|(t, _)| *t < self.map_size) // consider only applicable map based on part
                        .filter(|(t, _)| self.diagram[*t].is_none()) // only unoccupied spaces
                        .collect::<Vec<_>>()
                });

                let in_hallway = pos <= 6;
                let strangers_in_my_home = amphipod.home().iter().any(|&p| {
                    self.diagram[p]
                        .map(|other| !other.allowed_at(p))
                        .unwrap_or(false)
                });

                for (target, (_, steps)) in possible_targets.iter() {
                    let to_hallway = *target <= 6;
                    let to_a_home = !to_hallway;
                    let to_my_home = amphipod.home().contains(&target);

                    let legal = (!to_hallway || !in_hallway)
                        && (!to_a_home || (to_my_home && !strangers_in_my_home));

                    if legal {
                        let result = self.moved(pos, *target);
                        let cost = steps * amphipod.move_multiplier();
                        successors.push((result, cost));
                    }
                }
            }
        }

        successors
    }
}

impl std::fmt::Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("#############\n#")?;
        render(f, &self.diagram[0])?;
        render(f, &self.diagram[1])?;
        f.write_str(".")?;
        render(f, &self.diagram[2])?;
        f.write_str(".")?;
        render(f, &self.diagram[3])?;
        f.write_str(".")?;
        render(f, &self.diagram[4])?;
        f.write_str(".")?;
        render(f, &self.diagram[5])?;
        render(f, &self.diagram[6])?;
        f.write_str("#\n###")?;
        render(f, &self.diagram[7])?;
        f.write_str("#")?;
        render(f, &self.diagram[9])?;
        f.write_str("#")?;
        render(f, &self.diagram[11])?;
        f.write_str("#")?;
        render(f, &self.diagram[13])?;
        f.write_str("###\n  #")?;
        render(f, &self.diagram[8])?;
        f.write_str("#")?;
        render(f, &self.diagram[10])?;
        f.write_str("#")?;
        render(f, &self.diagram[12])?;
        f.write_str("#")?;
        render(f, &self.diagram[14])?;
        if self.map_size > 15 {
            f.write_str("#\n  #")?;
            render(f, &self.diagram[15])?;
            f.write_str("#")?;
            render(f, &self.diagram[17])?;
            f.write_str("#")?;
            render(f, &self.diagram[19])?;
            f.write_str("#")?;
            render(f, &self.diagram[21])?;
            f.write_str("#\n  #")?;
            render(f, &self.diagram[16])?;
            f.write_str("#")?;
            render(f, &self.diagram[18])?;
            f.write_str("#")?;
            render(f, &self.diagram[20])?;
            f.write_str("#")?;
            render(f, &self.diagram[22])?;
        }
        f.write_str("#\n  #########\n")
    }
}

impl std::fmt::Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amphipod::Amber => f.write_str("A"),
            Amphipod::Bronze => f.write_str("B"),
            Amphipod::Copper => f.write_str("C"),
            Amphipod::Desert => f.write_str("D"),
        }
    }
}

fn render(f: &mut std::fmt::Formatter<'_>, maybe_amphipod: &Option<Amphipod>) -> std::fmt::Result {
    match maybe_amphipod {
        None => f.write_str("."),
        Some(amphipod) => write!(f, "{}", amphipod),
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Part One

    // Example
    //let input = Burrow::for_part_one([
    //    B, C, B, D, // top row
    //    A, D, C, A, // bottom row
    //]);
    // My input
    let input = Burrow::for_part_one([
        B, B, C, D, // top row
        D, A, A, C, // bottom row
    ]);

    let goal = Burrow::for_part_one([
        A, B, C, D, // top row
        A, B, C, D, // bottom row
    ]);

    let (_, part_one_answer) = dijkstra(
        &input,
        |burrow| burrow.reachable(),
        |&burrow| burrow == goal,
    )
    .unwrap();
    dbg!(part_one_answer);

    // Part Two

    // Example
    //let input = Burrow::for_part_two([
    //    B, C, B, D,
    //    D, C, B, A,
    //    D, B, A, C,
    //    A, D, C, A,
    //]);
    // My input
    let input = Burrow::for_part_two([
        B, B, C, D, // top row
        D, C, B, A, // from instructions
        D, B, A, C, // from instructions
        D, A, A, C, // bottom row
    ]);

    let goal = Burrow::for_part_two([A, B, C, D, A, B, C, D, A, B, C, D, A, B, C, D]);

    let (_, part_two_answer) = dijkstra(
        &input,
        |burrow| burrow.reachable(),
        |&burrow| burrow == goal,
    )
    .unwrap();
    dbg!(part_two_answer);

    Ok(())
}
