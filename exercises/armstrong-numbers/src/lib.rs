pub fn is_armstrong_number(num: u32) -> bool {
    let digits = num.to_string();
    let len = digits.len();

    num == digits
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d.pow(len as u32))
        .sum()
}