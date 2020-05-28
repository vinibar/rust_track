use std::fmt;

// in minutes
const DAY: i32 = 1440;
const HOUR: i32 = 60;

pub struct Clock {
    minutes: u32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        Self { // the last + DAY) % DAY solves negative cases
            minutes: ((( (hours * HOUR) + minutes) % DAY) + DAY) % DAY
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}",  (self.minutes / 60 ), self.minutes % 60)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("minutes", &self.minutes)
            .finish()
    }
}

