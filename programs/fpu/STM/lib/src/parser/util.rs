use core::intrinsics::{cosf64, sinf64};

// Not an intrinsic for some reason
pub unsafe fn tanf64(value: f64) -> f64 {
    unsafe { sinf64(value) / cosf64(value) }
}
pub fn str_after(string: &str, index: usize) -> &str {
    if index >= string.len() {
        ""
    } else {
        &string[index..]
    }
}
