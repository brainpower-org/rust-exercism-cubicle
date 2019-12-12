#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum Throw {
    Strike,
    Spare,
    Pins(u16),
}

pub struct Frame {
    index: usize,
    first: Option<Throw>,
    second: Option<Throw>,
    fillball: Option<Throw>,
}

impl Frame {
    fn new(index: usize) -> Self {
        Frame {
            index: index
        }
    }
    
    fn isComplete(self) -> bool {
        let validThrows = self.first.is_some() && self.second.is_some();
        if self.index == 10 {
            validThrows &= self.fillball.is_some();
        }
        validThrows
    }

}

pub struct BowlingGame {
    frames: Vec<Frame>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {frames: vec![Frame {index: 1, first: None, second: None, fillball: None}]}
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
       if self.frames.len() > 10 {
          Err(Error::GameComplete)
       }
       else {
            let lastFrame = self.frames.last()
            if lastFrame.isComplete() {
                let current = Frame
            }
            Ok(())
       }
    }

    pub fn score(&self) -> Option<u16> {
        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
