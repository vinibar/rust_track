pub fn nth(n: u32) -> u32 {
    let mut result = 0;
    let mut prime_counter = 0;

    while prime_counter <= n {
        result += 1;
        if is_prime(result) {
            prime_counter += 1;
        }
    }
    result
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
