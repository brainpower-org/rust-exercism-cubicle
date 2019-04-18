/// Solution 1
/// ```
/// pub fn nth(n: u32) -> u32 {
///     let mut known_primes: Vec<u32> = vec![2];
/// 
///     for x in 3.. {
///         if known_primes.len() == (n + 1) as usize {
///             break;
///         }
/// 
///         if known_primes.iter().any(|y| x % y == 0) == false {
///             known_primes.push(x);
///         }
///     }
/// 
///     *known_primes.last().unwrap()
/// }
/// 
/// assert_eq!(nth(0), 2);
/// assert_eq!(nth(1), 3);
/// assert_eq!(nth(5), 13);
/// assert_eq!(nth(10000), 104743);
/// ```
/// 
pub fn nth(n: u32) -> u32 {
    let mut known_primes: Vec<u32> = vec![];

    (2..).find(|x| {
        if known_primes.iter().any(|y| x % y == 0) {
            false
        } else {
            known_primes.push(*x);
            known_primes.get(n as usize).is_some()
        }
    }).unwrap()
}
