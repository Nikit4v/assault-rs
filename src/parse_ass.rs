#![allow(dead_code)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: f32,
}

pub struct AssRow {
    layer: u32,
    start: Time,
    end: Time,
    style: String,
    name: String,
    margin_left: u32,
    margin_right: u32,
    margin_vertical: u32,
    effect: String,
    text: String,
}

#[allow(clippy::iter_nth_zero)]
impl From<String> for Time {
    fn from(s: String) -> Self {
        let l = s.split(':');
        Self {
            hour:   (l.clone().nth(0).unwrap().to_string()).parse::<u32>().unwrap(),
            minute: (l.clone().nth(1).unwrap().to_string()).parse::<u32>().unwrap(),
            second: (l.clone().nth(2).unwrap().to_string()).parse::<f32>().unwrap(),
        }
    }
}
