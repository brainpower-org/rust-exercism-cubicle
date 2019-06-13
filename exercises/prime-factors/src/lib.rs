pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let mut factor = 2;

    while n > 1 {
        if n % factor == 0 {
            result.push(factor);
            n = n / factor;
        } else {
            factor += 1;
        }
    }

    result
}
