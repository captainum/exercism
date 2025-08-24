#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hr = ((hours * 60 + minutes) / 60) % 24;

        if hours * 60 + minutes < 0 && minutes != 0 {
            hr -= 1;
        }
        
        if hr < 0 {
            hr += 24;
        }

        let mut mn = minutes % 60;
        if mn < 0 {
            mn += 60;
        }

        Clock {
            hours: hr,
            minutes: mn
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

use std::fmt;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}