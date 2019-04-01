use std::u64;

pub fn square(s: u32) -> u64 {
    match s {
        x if 1 <= x && x <= 64 => {
            let base: u64 = 2;
            base.pow(s - 1)
        }
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    u64::MAX
}
