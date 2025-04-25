use crate::gui::CalculatorState;

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
    (26, "sin("),
    (27, "cos("),
    (28, "tan("),
    (7, "X"),
    (1, "Y"),
    (31, "gamma("),
    (32, "^"),
    (33, "^2"),
    (34, "log("),
    (35, "e^")
];

// Keymap when in second mode
const SECOND_KEYMAP: &[(u8, &str)] = &[
    (25, ")"),
    (26, "arcsin("),
    (27, "arccos("),
    (28, "arctan("),
    (34, "ln("),
];

pub fn get_key_text(key: u8, state: &CalculatorState) -> Option<&'static str> {
    if state.second { SECOND_KEYMAP } else { NORMAL_KEYMAP }
        .iter()
        .find(|m| m.0 == key)
        .map(|x| x.1)
}
