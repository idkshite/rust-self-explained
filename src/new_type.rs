use std::fmt::{Display, Formatter};

struct TimeInSeconds(f64);

impl From<TimeInMinutes> for TimeInSeconds {
    fn from(minutes: TimeInMinutes) -> Self {
        Self(minutes.0 * 60.0)
    }
}

impl Display for TimeInSeconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
struct TimeInMinutes(f64);

fn minutes_to_hours(minutes: TimeInMinutes) -> f64{
    minutes.0 / 60.0
}

fn seconds_to_minutes(seconds: TimeInSeconds) -> f64{
    seconds.0 / 60.0
}

fn go_back_in_time(seconds: TimeInSeconds){
    // warp the time!
    println!("I warped the time backwards by {seconds} seconds");
}

pub(crate) fn main(){
    let seconds = TimeInSeconds(75.0);
    let minutes = TimeInMinutes(90.0);

    // let hours = minutes_to_hours(seconds);
    // 🚨 Type mismatch [E0308]
    // Expected:
    //     TimeInMinutes
    // Found:
    //     TimeInSeconds

    let converted_seconds = TimeInSeconds::from(minutes);

    go_back_in_time(converted_seconds);
}