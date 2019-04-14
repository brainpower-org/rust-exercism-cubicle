/// Solution 1
/// ```rust
/// pub fn raindrops(n: u32) -> String {
///    let mut result = String::new();
///
///    if n % 3 == 0 {
///        result.push_str("Pling");
///    }
///
///    if n % 5 == 0 {
///        result.push_str("Plang");
///    }
///
///    if n % 7 == 0 {
///        result.push_str("Plong");
///    }
///
///    if result.is_empty() {
///        n.to_string()
///    } else {
///        result
///    }
/// }
///
/// assert_eq!("1", raindrops::raindrops(1));
/// assert_eq!("Pling", raindrops::raindrops(3));
/// assert_eq!("Plang", raindrops::raindrops(5));
/// assert_eq!("Plong", raindrops::raindrops(7));
/// assert_eq!("Pling", raindrops::raindrops(6));
/// assert_eq!("8", raindrops::raindrops(8));
/// assert_eq!("Pling", raindrops::raindrops(9));
/// assert_eq!("Plang", raindrops::raindrops(10));
/// assert_eq!("Plong", raindrops::raindrops(14));
/// assert_eq!("PlingPlang", raindrops::raindrops(15));
/// assert_eq!("PlingPlong", raindrops::raindrops(21));
/// assert_eq!("Plang", raindrops::raindrops(25));
/// assert_eq!("Pling", raindrops::raindrops(27));
/// assert_eq!("PlangPlong", raindrops::raindrops(35));
/// assert_eq!("Plong", raindrops::raindrops(49));
/// assert_eq!("52", raindrops::raindrops(52));
/// assert_eq!("PlingPlangPlong", raindrops::raindrops(105));
/// assert_eq!("Plang", raindrops::raindrops(3125));
/// assert_eq!("12121", raindrops::raindrops(12121));
/// ```
///
/// Solution 2
/// ```rust
/// pub fn raindrops(n: u32) -> String {
///    let result = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
///        .into_iter()
///        .filter_map(|(f, s)| if n % f == 0 { Some(s) } else { None })
///        .flat_map(|x| x.chars())
///        .collect::<String>();
///
///    if result.is_empty() {
///        n.to_string()
///    } else {
///        result
///    }
/// }
///
/// assert_eq!("1", raindrops::raindrops(1));
/// assert_eq!("Pling", raindrops::raindrops(3));
/// assert_eq!("Plang", raindrops::raindrops(5));
/// assert_eq!("Plong", raindrops::raindrops(7));
/// assert_eq!("Pling", raindrops::raindrops(6));
/// assert_eq!("8", raindrops::raindrops(8));
/// assert_eq!("Pling", raindrops::raindrops(9));
/// assert_eq!("Plang", raindrops::raindrops(10));
/// assert_eq!("Plong", raindrops::raindrops(14));
/// assert_eq!("PlingPlang", raindrops::raindrops(15));
/// assert_eq!("PlingPlong", raindrops::raindrops(21));
/// assert_eq!("Plang", raindrops::raindrops(25));
/// assert_eq!("Pling", raindrops::raindrops(27));
/// assert_eq!("PlangPlong", raindrops::raindrops(35));
/// assert_eq!("Plong", raindrops::raindrops(49));
/// assert_eq!("52", raindrops::raindrops(52));
/// assert_eq!("PlingPlangPlong", raindrops::raindrops(105));
/// assert_eq!("Plang", raindrops::raindrops(3125));
/// assert_eq!("12121", raindrops::raindrops(12121));
/// ```
pub fn raindrops(n: u32) -> String {
    // Solution 3
    [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .map(|(f, s)| {
            if n % f == 0 {
                Some(s.to_string())
            } else {
                None
            }
        })
        .fold(None as Option<String>, map_strings)
        .map_or(n.to_string(), |x| x)
}

fn map_strings(acc: Option<String>, item: Option<String>) -> Option<String> {
    match (acc, item) {
        (Some(mut a), Some(b)) => {
            a.push_str(&b);
            Some(a)
        }
        (a, b) => a.or(b),
    }
}
