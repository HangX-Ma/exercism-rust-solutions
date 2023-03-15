use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let mut total_minutes = (hours * 60 + minutes) % (24 * 60);
        if total_minutes < 0 {total_minutes += 24 * 60};
        Clock { hours: total_minutes / 60, minutes: total_minutes % 60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // unimplemented!("Add {minutes} minutes to existing Clock time");
        Self::new(self.hours, self.minutes + minutes)
    }
}
