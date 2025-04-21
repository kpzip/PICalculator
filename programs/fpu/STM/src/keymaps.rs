// Keymap when not in alpha or second
const NORMAL_KEYMAP: &[(u8, &str)] = &[
    (2, "."),
    (3, "0"),
    (8, "7"),
    (9, "8"),
    (10, "9"),
    (14, "4"),
    (15, "5"),
    (16, "6"),
    (20, "1"),
    (21, "2"),
    (22, "3"),
    (29, "+"),
    (23, "-"),
    (17, "*"),
    (11, "/"),
    (25, "("),
];

// Keymap when in second mode
const SECOND_KEYMAP: &[(u8, &str)] = &[(25, ")")];

pub fn get_key_text(key: u8, second: bool, alpha: bool) -> Option<&'static str> {
    if second { SECOND_KEYMAP } else { NORMAL_KEYMAP }
        .iter()
        .find(|m| m.0 == key)
        .map(|x| x.1)
}
