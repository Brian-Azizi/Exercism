#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hour, minute) = unwind_clock(hours, minutes);
        Clock { hour, minute }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hour, self.minute + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:0>2}:{:0>2}", self.hour, self.minute)
    }
}

fn unwind_clock(hours: i32, minutes: i32) -> (i32, i32) {
    let mut hour = hours;
    let mut minute = minutes;

    while minute < 0 {
        hour -= 1;
        minute += 60;
    }

    while minute >= 60 {
        hour += 1;
        minute -= 60;
    }

    while hour < 0 {
        hour += 24
    }

    while hour >= 24 {
        hour -= 24
    }

    return (hour, minute);
}
