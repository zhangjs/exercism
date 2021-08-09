#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    second: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: vec![],
            second: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || self.second && pins + self.rolls.last().unwrap() > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(Error::GameComplete)
        } else {
            self.rolls.push(pins);
            self.second = if pins != 10 { !self.second } else { false };

            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut total = 0;
        let mut frame = 0;
        let rolls = &self.rolls;

        for _ in 0..10 {
            if let (Some(&first), Some(second)) = (rolls.get(frame), rolls.get(frame + 1)) {
                total += first + second;
                if first == 10 || first + second == 10 {
                    if let Some(&third) = rolls.get(frame + 2) {
                        total += third;
                    } else {
                        return None;
                    }
                }

                frame += if first == 10 { 1 } else { 2 };
            } else {
                return None;
            }
        }

        Some(total)
    }
}
