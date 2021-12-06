use color_eyre::eyre::Result;
use reformation::Reformation;

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;

    // index = timer value
    let mut population = [0usize; 9];
    input
        .split(",")
        .map(|s| usize::parse(s).unwrap() )
        .for_each(|n| population[n] += 1);

    // Part One
    for _ in 0..80 {
        population[..].rotate_left(1);
        population[6] += population[8];
    }
    println!("first answer: {}", population.iter().sum::<usize>());

    // Part Two
    for _ in 80..256 {
        population[..].rotate_left(1);
        population[6] += population[8];
    }
    println!("second answer: {}", population.iter().sum::<usize>());

    Ok(())
}

#[allow(dead_code)]
const EXAMPLE: &'static str = "3,4,3,1,2";

const INPUT: &'static str = "2,1,2,1,5,1,5,1,2,2,1,1,5,1,4,4,4,3,1,2,2,3,4,1,1,5,1,1,4,2,5,5,5,1,1,4,5,4,1,1,4,2,1,4,1,2,2,5,1,1,5,1,1,3,4,4,1,2,3,1,5,5,4,1,4,1,2,1,5,1,1,1,3,4,1,1,5,1,5,1,1,5,1,1,4,3,2,4,1,4,1,5,3,3,1,5,1,3,1,1,4,1,4,5,2,3,1,1,1,1,3,1,2,1,5,1,1,5,1,1,1,1,4,1,4,3,1,5,1,1,5,4,4,2,1,4,5,1,1,3,3,1,1,4,2,5,5,2,4,1,4,5,4,5,3,1,4,1,5,2,4,5,3,1,3,2,4,5,4,4,1,5,1,5,1,2,2,1,4,1,1,4,2,2,2,4,1,1,5,3,1,1,5,4,4,1,5,1,3,1,3,2,2,1,1,4,1,4,1,2,2,1,1,3,5,1,2,1,3,1,4,5,1,3,4,1,1,1,1,4,3,3,4,5,1,1,1,1,1,2,4,5,3,4,2,1,1,1,3,3,1,4,1,1,4,2,1,5,1,1,2,3,4,2,5,1,1,1,5,1,1,4,1,2,4,1,1,2,4,3,4,2,3,1,1,2,1,5,4,2,3,5,1,2,3,1,2,2,1,4";
