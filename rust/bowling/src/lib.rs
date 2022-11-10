#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<u16>,

}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { frames: vec![] }
    }

    pub fn is_game_completed(&self) -> bool {
        if self.frames.len() < 20 {
            return false;
        }

        let (_, last_two_rolls) = self.frames.split_at(18);
        if last_two_rolls[0] == 10 {
            self.frames.len() == 22
        } else if last_two_rolls[0] + last_two_rolls[1] == 10 {
            self.frames.len() == 21
        } else {
            true
        } 
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_completed() {
            return Err(Error::GameComplete);
        }

        let mut pins_left = 10;

        if self.frames.len() % 2 == 1 && self.frames.len() < 20{
            pins_left -= self.frames.last().unwrap();
        }  else if self.frames.len() == 21 && self.frames[20] != 10{   
            pins_left -= self.frames.last().unwrap();
        }


        if pins_left < pins {
            return  Err(Error::NotEnoughPinsLeft);
        }

        self.frames.push(pins);

        if pins == 10 && self.frames.len() % 2 == 1 && self.frames.len() < 20{
            self.frames.push(0);
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        println!("{:?}", self.frames);
        if !self.is_game_completed() {
            return None;
        }

        let mut score = 0;
        for i in (0..20).step_by(2) {
            score += self.frames[i];
            score += self.frames[i + 1];

            if self.frames[i] == 10 {
                if self.frames[i + 2] == 10 && i != 18 {
                    score += self.frames[i + 2];
                    score += self.frames[i + 4];
                } else {
                    score += self.frames[i + 3];
                    score += self.frames[i + 2];
                }
            } else if self.frames[i] + self.frames[i + 1] == 10 {
                score += self.frames[i + 2];
            }
        }

        Some(score)
    }
}
