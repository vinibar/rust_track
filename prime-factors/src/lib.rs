// stolen from https://exercism.io/tracks/rust/exercises/prime-factors/solutions/88ab0a07b0a044d98a429727775a7d41
pub fn factors(n: u64) -> Vec<u64> {
    let mut number = n;
    let mut factors = vec![];
    let mut candidates = 2..;

    while number > 1 {
        let x = candidates.next().unwrap();

        while number % x == 0 {
            number /= x;
            factors.push(x);
        }
    }

    factors
}
