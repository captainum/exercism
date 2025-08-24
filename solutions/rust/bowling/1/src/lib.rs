#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct State {
    rolls_left: u8,
    score: u16,
    is_count: bool,
}

pub struct BowlingGame {
    frame: u8,
    is_second_roll: bool,
    score: u16,
    fill_ball: u8,

    pins_left: u16,

    states: Vec<State>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frame: 1, is_second_roll: false, score: 0, fill_ball: 0 , pins_left: 10, states: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.pins_left {
            Err(Error::NotEnoughPinsLeft)
        }
        else if self.frame > 10 && self.fill_ball == 0 {
            Err(Error::GameComplete)
        }
        else {
            if self.fill_ball > 0 {
                self.score += pins;
                self.fill_ball -= 1;

                self.pins_left -= pins;

                if self.pins_left == 0 {
                    self.pins_left = 10;
                }
            }
            else if pins == self.pins_left {
                if self.frame < 10 {
                    self.states.push(
                        State {
                            rolls_left: if pins == 10 { 2 } else { 1 }, score: 10, is_count: false,
                        }
                    );
                }
                else {
                    self.score += 10;
                    self.fill_ball = if pins == 10 { 2 } else { 1 };
                }

                self.frame += 1;
                self.pins_left = 10;

                self.is_second_roll = false;
            }
            else if self.is_second_roll {
                self.score += 10 - (self.pins_left - pins);

                self.frame += 1;
                self.is_second_roll = false;
                self.pins_left = 10;
            }
            else {
                self.is_second_roll = true;
                self.pins_left -= pins;
            }

            let mut cnt = 0;
            for state in &mut self.states {
                if state.is_count {
                    state.rolls_left -= 1;
                    state.score += pins;

                    if state.rolls_left == 0 {
                        cnt += 1;
                    }
                }
                else { state.is_count = true; }
            }

            while cnt > 0 {
                let state = self.states.remove(0);
                println!("pushing {}", state.score);
                self.score += state.score;
                cnt -= 1;
            }

            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        println!("score {}, frame {}, fb {}", self.score, self.frame, self.fill_ball);
        if self.frame == 11 && self.fill_ball == 0 {
            Some(self.score)
        }
        else { None }
    }
}