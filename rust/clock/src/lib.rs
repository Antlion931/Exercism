use std::fmt::Display;

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY:i32 = 24;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    minutes:i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { minutes: (hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY)}
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
