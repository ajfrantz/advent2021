use color_eyre::eyre::{bail, eyre, Result};
use reformation::Reformation;

use rustc_hash::FxHashMap;

#[derive(Debug, Reformation, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Instruction {
    #[reformation("inp {}")]
    Input(Operand),
    #[reformation("add {} {}")]
    Add(Operand, Operand),
    #[reformation("mul {} {}")]
    Mul(Operand, Operand),
    #[reformation("div {} {}")]
    Div(Operand, Operand),
    #[reformation("mod {} {}")]
    Mod(Operand, Operand),
    #[reformation("eql {} {}")]
    Eql(Operand, Operand),
}

#[derive(Debug, Reformation, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operand {
    #[reformation("w")]
    W,
    #[reformation("x")]
    X,
    #[reformation("y")]
    Y,
    #[reformation("z")]
    Z,
    #[reformation("{}")]
    Literal(i64),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl ALU {
    fn get(&self, operand: Operand) -> i64 {
        match operand {
            Operand::W => self.w,
            Operand::X => self.x,
            Operand::Y => self.y,
            Operand::Z => self.z,
            Operand::Literal(v) => v,
        }
    }

    fn set(&mut self, operand: Operand, v: i64) {
        match operand {
            Operand::W => self.w = v,
            Operand::X => self.x = v,
            Operand::Y => self.y = v,
            Operand::Z => self.z = v,
            Operand::Literal(_) => panic!("cannot store into literal"),
        }
    }

    fn run(&self, instruction: Instruction, input: Option<i64>) -> Result<ALU> {
        let mut alu = self.clone();

        match instruction {
            Instruction::Input(a) => {
                alu.set(a, input.ok_or_else(|| eyre!("should have had input"))?);
            }
            Instruction::Add(a, b) => alu.set(a, alu.get(a) + alu.get(b)),
            Instruction::Mul(a, b) => alu.set(a, alu.get(a) * alu.get(b)),
            Instruction::Div(a, b) => {
                if alu.get(b) == 0 {
                    bail!("div by 0");
                }
                alu.set(a, alu.get(a) / alu.get(b));
            }
            Instruction::Mod(a, b) => {
                if alu.get(a) < 0 || alu.get(b) <= 0 {
                    bail!("illegal mod");
                }
                alu.set(a, alu.get(a) % alu.get(b));
            }
            Instruction::Eql(a, b) => alu.set(a, (alu.get(a) == alu.get(b)) as i64),
        }

        Ok(alu)
    }
}

#[allow(dead_code)]
fn handle_digit(input: i64, mut z: i64, divisor: i64, addend1: i64, addend2: i64) -> i64 {
    // inp w
    // (we'll just call this input)

    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;

    // div z {}
    z = z / divisor;

    let x = x + addend1; // add x {}

    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // and also
    // mul y 0
    // add y w
    // add y {}
    // mul y x
    // add z y
    if x != input {
        z *= 26;
        z += input + addend2;
    }

    z
}

#[allow(dead_code)]
fn validator(_input: [i64; 14]) -> bool {
    unimplemented!();

    //let mut z = 0;
    //z = handle_digit(input[0], 1, 12, 1);
    // push input[0] + 1
    //z = handle_digit(input[1], 1, 12, 1);
    // push input[1] + 1
    //z = handle_digit(input[2], 1, 15, 16);
    // push input[2] + 16
    //z = handle_digit(input[3], 26, -8, 5);
    // input[3] = (pop) - 8
    //z = handle_digit(input[4], 26, -4, 9);
    // input[4] = (pop) - 4
    //z = handle_digit(input[5], 1, 15, 3);
    // push input[5] + 3
    //z = handle_digit(input[6], 1, 14, 2);
    // push input[6] + 2
    //z = handle_digit(input[7], 1, 14, 15);
    // push input[7] + 15
    //z = handle_digit(input[8], 26, -13, 5);
    // input[8] = (pop) - 13
    //z = handle_digit(input[9], 26, -3, 11);
    // input[9] = (pop) - 3
    //z = handle_digit(input[10], 26, -7, 7);
    // input[10] = (pop) - 7
    //z = handle_digit(input[11], 1, 10, 1);
    // push input[11] + 1
    //z = handle_digit(input[12], 26, -6, 10);
    // input[12] = (pop) - 6
    //z = handle_digit(input[13], 26, -8, 3);
    // input[13] = (pop) - 8
    //
    //z == 0
    //
    // input[3] = input[2] + 8
    // input[4] = input[1] - 3
    // input[8] = input[7] + 2
    // input[9] = input[6] - 1
    // input[10] = input[5] - 4
    // input[12] = input[11] - 5
    // input[13] = input[0] - 7
}

fn part_one() -> i64 {
    99_196_997_985_942
}

fn part_two() -> i64 {
    84_191_521_311_611
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Being "smart"
    // (aka having a lot of time to reconsider my life decisions while brute force finishes)
    // While decompiling things into source I realized it's checking one digit at a time, against
    // some previous digit, using the 'z' register as a sort of stack.
    // From that we can hand-compute the constraints on each digit, and from *that* we can just
    // hand compute the max and min directly.
    dbg!(part_one());
    dbg!(part_two());

    // Brute force
    // I left this because it *does* seem to work. It's rather slow and will munch a bunch of RAM.
    let mut states: FxHashMap<ALU, i64> = FxHashMap::default();
    states.insert(ALU::default(), 0);

    for (i, line) in INPUT.split("\n").enumerate() {
        let instruction = Instruction::parse(line)?;
        match instruction {
            Instruction::Input(_) => {
                // Split a new state off for every possible input digit.
                let mut new_states = FxHashMap::default();
                new_states.reserve(9 * states.len());
                for digit in 1..=9 {
                    for (alu, input) in &states {
                        // If this instruction results in a valid state...
                        if let Ok(alu) = alu.run(instruction, Some(digit)) {
                            let input = 10 * input + digit;
                            new_states.insert(alu, input);
                        }
                    }
                }
                states = new_states;
            }

            _ => {
                // Just kick each machine along to the next state, dropping ones that fail.
                let mut new_states = FxHashMap::default();
                new_states.reserve(9 * states.len());

                for (alu, input) in states.into_iter() {
                    if let Ok(alu) = alu.run(instruction, None) {
                        let entry = new_states.entry(alu).or_insert(input);
                        *entry = (*entry).max(input);
                    }
                }

                states = new_states;
            }
        }

        eprintln!(
            "after instruction {}, tracking {} states",
            i + 1,
            states.len()
        );
    }

    eprintln!("filtering to accepted model numbers numbers");
    states.retain(|alu, _| alu.z == 0);
    eprintln!("now just {} states", states.len());

    dbg!(states.values().max());
    dbg!(states.values().min());

    Ok(())
}

const INPUT: &'static str = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 16
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -6
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y";
