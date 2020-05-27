pub fn square(s: u32) -> u64 {
    (1..65)
        .position(|n| n == s)
        .expect("Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u128 {
    2u128.pow(64) - 1
}
