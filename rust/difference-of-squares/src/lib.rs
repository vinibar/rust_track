pub fn square_of_sum(n: u32) -> u32 {
    let total: u32 = n * (n + 1) / 2;
    total * total
}

pub fn sum_of_squares(n: u32) -> u32 {
    let total: u32 = n * (n + 1) * (2 * n + 1);
    total / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
