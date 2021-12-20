use color_eyre::eyre::Result;
use reformation::Reformation;

#[derive(Debug, Reformation)]
#[reformation("target area: x={x_min}..{x_max}, y={y_min}..{y_max}")]
struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn contains(&self, (x, y): (i32, i32)) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

fn step(pos: &mut (i32, i32), vel: &mut (i32, i32)) {
    pos.0 += vel.0;
    pos.1 += vel.1;
    if vel.0 != 0 {
        vel.0 -= vel.0 / vel.0.abs();
    }
    vel.1 -= 1;
}

#[derive(Debug)]
struct Trajectory {
    initial_velocity: (i32, i32),
    max_height: i32,
}

impl Trajectory {
    fn simulate(initial_velocity: (i32, i32), target: &Target) -> Option<Trajectory> {
        let mut pos = (0, 0);
        let mut vel = initial_velocity;
        let mut max_height = 0;

        while !target.contains(pos) {
            if pos.1 < target.y_min {
                return None;
            }
            step(&mut pos, &mut vel);
            max_height = max_height.max(pos.1);
        }

        Some(Trajectory {
            initial_velocity,
            max_height,
        })
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = INPUT;

    let target = Target::parse(input).unwrap();

    let mut trajectories = Vec::new();
    for x_vel in 0..=target.x_max {
        for y_vel in target.y_min..1000 {
            if let Some(t) = Trajectory::simulate((x_vel, y_vel), &target) {
                trajectories.push(t);
            }
        }
    }
    dbg!(trajectories.len());
    dbg!(trajectories.iter().max_by_key(|t| t.max_height));

    Ok(())
}

#[allow(dead_code)]
const EXAMPLE: &'static str = "target area: x=20..30, y=-10..-5";

const INPUT: &'static str = "target area: x=138..184, y=-125..-71";
