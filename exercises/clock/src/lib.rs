#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32
}

const DAY_HOURS: i32 = 24 * 60;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = ((hours * 60 + minutes) % DAY_HOURS + DAY_HOURS) % DAY_HOURS;
        Clock { minutes }
    }

    fn hours(&self) -> i32 {
        self.minutes / 60
    }

    fn minutes(&self) -> i32 {
        self.minutes % 60
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours(), self.minutes() + minutes)
    }
}

use std::fmt;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
