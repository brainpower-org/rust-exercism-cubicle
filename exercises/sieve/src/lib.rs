use std::collections::HashMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let vec: Vec<u64> = Vec::new();
    let mut primes: HashMap<u64, bool> = HashMap::new();

    if upper_bound < 2 {
        return vec;
    }

    (2..=upper_bound).for_each(|c| {
        primes.insert(c, true);
    });

    (2..=upper_bound).for_each(|number| {
        if !*primes.get(&number).unwrap() {
            return ();
        }

        for x in ((number * 2)..=upper_bound).step_by(number as usize) {
            primes.insert(x, false);
        }
    });

    let mut result = primes
        .iter()
        .filter_map(|(number, prime)| {
            if *prime {
                return Some(*number);
            }

            None
        })
        .collect::<Vec<u64>>();
    result.sort();
    result
        
}
