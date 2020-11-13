use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        let clock1 = self.norm();
        let clock2 = other.norm();
        clock1.hour == clock2.hour && clock1.minute == clock2.minute
    }
}

impl Eq for Clock {}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Clock {
            hour: hours,
            minute: minutes,
        };

        clock.norm()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let clock = Clock {
            hour: self.hour,
            minute: self.minute + minutes,
        };

        clock.norm()
    }

    fn norm(&self) -> Self {
        let mut h = (self.hour + self.minute / 60) % 24;
        let mut m = self.minute % 60;

        if m < 0 {
            m += 60;
            h -= 1;
        }

        if h < 0 {
            h += 24;
        }

        Clock { hour: h, minute: m }
    }
}
