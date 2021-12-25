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
    text: String
}

impl From<String> for Time {
    fn from(s: String) -> Self {
        let l = s.split(":");
        Self {
            hour: (l[0] as String).parse::<u32>().unwrap(),
            minute: (l[1] as String).parse::<u32>().unwrap(),
            second: (l[2] as String).parse::<f32>().unwrap(),
        }
    }
}




