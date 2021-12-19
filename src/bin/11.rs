use color_eyre::eyre::Result;

use advent2021::grid::{Grid, Neighbors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Octopus {
    Charging(u8),
    Flashed,
}

impl Default for Octopus {
    fn default() -> Self {
        Octopus::Charging(0)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;

    let mut grid: Grid<Octopus> = Grid::new(0, 10, 0, 10);
    for (row, line) in input.split("\n").enumerate() {
        for (col, charge) in line.bytes().enumerate() {
            grid[(col as i32, row as i32)] = Octopus::Charging(charge - b'0');
        }
    }

    let mut flashes = 0;
    for step in 1.. {
        // Everyone gains 1 charge to start.
        grid.for_each_mut(|octopus| {
            if let Octopus::Charging(c) = octopus {
                *octopus = Octopus::Charging(*c + 1);
            }
        });

        // Flash anyone who's exceeded 9 charge until we stop flashing.
        let mut flashed = true;
        while flashed {
            flashed = false;
            for cell in grid.cells() {
                match grid[cell] {
                    Octopus::Charging(c) if c > 9 => {
                        flashed = true;
                        flashes += 1;
                        grid[cell] = Octopus::Flashed;

                        // A flash adds one to all neighbors charge.
                        for neighbor in cell.neighbors() {
                            let neighbor = grid.get_mut(neighbor);
                            match neighbor {
                                Some(Octopus::Charging(c)) => {
                                    *neighbor.unwrap() = Octopus::Charging(*c + 1);
                                }
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        // Part One
        if step == 100 {
            dbg!(flashes);
        }

        // Part Two
        if grid.cells().all(|coord| grid[coord] == Octopus::Flashed) {
            dbg!(step);
            break;
        }

        // Finally, reset all flashed octopuses to 0.
        grid.for_each_mut(|octopus| {
            if let Octopus::Flashed = octopus {
                *octopus = Octopus::Charging(0);
            }
        });
    }

    Ok(())
}

#[allow(dead_code)]
const EXAMPLE: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const INPUT: &'static str = "8258741254
3335286211
8468661311
6164578353
2138414553
1785385447
3441133751
3586862837
7568272878
6833643144";
