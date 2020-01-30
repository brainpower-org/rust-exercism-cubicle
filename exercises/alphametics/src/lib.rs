use std::collections::HashMap;
use std::iter::FromIterator;

// "I + BB == ILL"
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let clean_input = input.replace(" ", "");
    let useable_characters: HashMap<char, Vec<u8>> = clean_input.chars()
        .filter(|c| c.is_alphabetic())
        .into_iter()
        .map(|c| (c, vec![0,1,2,3,4,5,6,7,8,9]))
        .collect();

    let splitted_input = clean_input.split("==").collect::<Vec<&str>>();
    let leftside = splitted_input.get(0).unwrap().split("+").collect::<Vec<&str>>();
    let rightside = splitted_input.get(1).unwrap();


    dbg!((leftside, rightside));
    None
}
