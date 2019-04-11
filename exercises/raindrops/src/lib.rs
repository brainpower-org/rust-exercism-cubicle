pub fn raindrops(n: u32) -> String {
    // Solution 1
    // let mut result = String::new();

    // if n % 3 == 0 {
    //     result.push_str("Pling");
    // }

    // if n % 5 == 0 {
    //     result.push_str("Plang");
    // }

    // if n % 7 == 0 {
    //     result.push_str("Plong");
    // }

    // if result.is_empty() {
    //     n.to_string()
    // } else {
    //     result
    // }

    // Solution 2
    let result = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter_map(|(f, s)| if n % f == 0 { Some(s) } else { None })
        .flat_map(|x| x.chars())
        .collect::<String>();

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
