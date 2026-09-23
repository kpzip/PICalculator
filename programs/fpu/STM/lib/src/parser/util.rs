const PI: f64 = core::f64::consts::PI;

pub fn to_radians(value: f64) -> f64 {
    value * PI / 180.0
}

pub fn str_after(string: &str, index: usize) -> &str {
    if index >= string.len() {
        ""
    } else {
        &string[index..]
    }
}
