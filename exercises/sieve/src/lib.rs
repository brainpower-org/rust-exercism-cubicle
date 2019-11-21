pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound).fold(
        (2..=upper_bound).into_iter().collect::<Vec<u64>>(),
        |mut acc, number| {
            for x in ((number * 2)..=upper_bound).step_by(number as usize) {
                acc = acc.into_iter().filter(|c| *c != x).collect();
            }
            acc
        },
    )
}
