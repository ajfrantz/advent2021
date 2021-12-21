use color_eyre::eyre::Result;

#[derive(Debug, Clone, Copy)]
struct Player {
    position: usize,
    score: usize,
}

#[derive(Debug, Clone, Copy)]
struct Game {
    players: [Player; 2],
    current_player: usize,
    rounds_played: usize,
    win_at: usize,
}

impl Game {
    fn new(initial_positions: [usize; 2], win_at: usize) -> Game {
        Game {
            players: [
                Player {
                    position: initial_positions[0],
                    score: 0,
                },
                Player {
                    position: initial_positions[1],
                    score: 0,
                },
            ],
            current_player: 0,
            rounds_played: 0,
            win_at,
        }
    }

    fn do_turn<R>(&mut self, rolls: R) -> bool
    where
        R: Fn(usize) -> usize,
    {
        let round = self.rounds_played;
        let rolls = rolls(round);
        self.rounds_played += 1;

        let player = &mut self.players[self.current_player];
        player.position = ((player.position - 1) + rolls) % 10 + 1;
        player.score += player.position;
        if player.score >= self.win_at {
            return true;
        }

        self.current_player ^= 1;
        return false;
    }

    fn part_one(&mut self) {
        let deterministic_dice = |round| {
            let mut rolls = (3 * round) % 100 + 1;
            rolls += (3 * round + 1) % 100 + 1;
            rolls += (3 * round + 2) % 100 + 1;
            rolls
        };
        while !self.do_turn(deterministic_dice) {}

        let loser = self.current_player ^ 1;
        let loser = &self.players[loser];
        dbg!(loser.score * self.rounds_played * 3);
    }

    fn part_two(&mut self) -> [usize; 2] {
        let mut wins = [0; 2];

        // (rolled value, frequency)
        const ROLLS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        for (roll_value, roll_frequency) in &ROLLS {
            let mut copy = self.clone();
            let roll = |_| *roll_value;
            if copy.do_turn(roll) {
                wins[copy.current_player] += roll_frequency;
            } else {
                for (player, new_wins) in copy.part_two().iter().enumerate() {
                    wins[player] += new_wins * roll_frequency;
                }
            }
        }

        wins
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Example
    let mut game = Game::new([4, 8], 1000);
    game.part_one();

    // Part One
    let mut game = Game::new([8, 1], 1000);
    game.part_one();

    // Part Two
    let mut game = Game::new([8, 1], 21);
    dbg!(game.part_two());

    Ok(())
}
