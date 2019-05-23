/// # Solution 1
/// ```
/// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
///     (1..limit)
///         .filter(|x| factors.iter().filter(|y| *y > &0).any(|y| x % y == 0))
///         .sum()
/// }
///
/// # assert_eq!(0, sum_of_multiples(1, &[3, 5]));
/// # assert_eq!(3, sum_of_multiples(4, &[3, 5]));
/// # assert_eq!(9, sum_of_multiples(7, &[3]));
/// # assert_eq!(23, sum_of_multiples(10, &[3, 5]));
/// # assert_eq!(2318, sum_of_multiples(100, &[3, 5]));
/// # assert_eq!(233168, sum_of_multiples(1000, &[3, 5]));
/// # assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]));
/// # assert_eq!(30, sum_of_multiples(15, &[4, 6]));
/// # assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]));
/// # assert_eq!(275, sum_of_multiples(51, &[5, 25]));
/// # assert_eq!(2203160, sum_of_multiples(10000, &[43, 47]));
/// # assert_eq!(4950, sum_of_multiples(100, &[1]));
/// # assert_eq!(0, sum_of_multiples(10000, &[]));
/// # assert_eq!(0, sum_of_multiples(1, &[0]));
/// # assert_eq!(3, sum_of_multiples(4, &[3, 0]));
/// # assert_eq!(39614537, sum_of_multiples(10000, &[2, 3, 5, 7, 11]));
/// ```
///
/// # Solution 2
/// ```
/// use std::collections::HashSet;
///
/// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
///    let mut result = HashSet::new();
///    
///    for x in 1..limit {
///            for factor in factors {
///                 if *factor > 0 && x % factor == 0 {
///                     result.insert(x);
///                 }    
///            }
///    }
///
///    result.iter().sum()
/// }
///
/// # assert_eq!(0, sum_of_multiples(1, &[3, 5]));
/// # assert_eq!(3, sum_of_multiples(4, &[3, 5]));
/// # assert_eq!(9, sum_of_multiples(7, &[3]));
/// # assert_eq!(23, sum_of_multiples(10, &[3, 5]));
/// # assert_eq!(2318, sum_of_multiples(100, &[3, 5]));
/// # assert_eq!(233168, sum_of_multiples(1000, &[3, 5]));
/// # assert_eq!(51, sum_of_multiples(20, &[7, 13, 17]));
/// # assert_eq!(30, sum_of_multiples(15, &[4, 6]));
/// # assert_eq!(4419, sum_of_multiples(150, &[5, 6, 8]));
/// # assert_eq!(275, sum_of_multiples(51, &[5, 25]));
/// # assert_eq!(2203160, sum_of_multiples(10000, &[43, 47]));
/// # assert_eq!(4950, sum_of_multiples(100, &[1]));
/// # assert_eq!(0, sum_of_multiples(10000, &[]));
/// # assert_eq!(0, sum_of_multiples(1, &[0]));
/// # assert_eq!(3, sum_of_multiples(4, &[3, 0]));
/// # assert_eq!(39614537, sum_of_multiples(10000, &[2, 3, 5, 7, 11]));
/// ```

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
        let mut result = vec![];
        let mut sum = 0;

        for x in 1..limit {
                for factor in factors {
                        if *factor > 0 && x % factor == 0 && !result.contains(&x) {
                                result.push(x);
                                sum += x;
                        }
                }
        }

        sum
}
