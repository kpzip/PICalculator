use alloc::string::String;

pub fn truncate_trailing_zeros(mut string: String) -> String {
    for i in (0..string.len()).rev() {
        if string.as_bytes()[i] != '0' as u8 {
            if string.as_bytes()[i] == '.' as u8 {
                string.truncate(i);
            } else {
                string.truncate(i + 1);
            }
            break;
        }
    }
    string
}

pub fn abs(val: i8) -> i8 {
    if val < 0 {
        -val
    } else {
        val
    }
}
