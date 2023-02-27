use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        if minutes < 0 {
            Self::new(hours - 1, minutes + 60)
        } else if hours < 0 {
            Self::new(hours + 24, minutes)
        } else {
            Self {
                hour: (hours + minutes / 60) % 24,
                minute: minutes % 60,
            }
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hour, self.minute + minutes)
    }
}
