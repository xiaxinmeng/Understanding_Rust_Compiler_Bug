rust
//#![feature(nll)]

type Roll = u8;
type Score = u16;

pub struct Game {
    rolls: Vec<Roll>,
}

impl Game {
    pub fn new() -> Self {
        Game { rolls: Vec::<Roll>::new() }
    }

    pub fn roll(&mut self, roll: Roll) -> &mut Self {
        self.rolls.push(roll);
        self
    }

    pub fn score(&self) -> Option<Score> {
        match self.rolls.len() {
            0 => None,
            _ => Some(self.rolls.iter()
                                .map(|&v| Score::from(v))
                                .sum())
        }
    }
}

fn main() {
    // GIVEN a Game instance
    let expected_result = Some(0 as Score);
    let rolls = [0 as Roll];
    let mut game = Game::new();
    let mut sut = |v| game.roll(v);
    
    // WHEN a gutterball is rolled
    rolls.iter()
         .for_each(|&v| { sut(v); } );
    
    // THEN score should be 0
    assert!(game.score() == expected_result);
}
