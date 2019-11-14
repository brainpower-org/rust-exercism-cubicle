pub fn abbreviate(phrase: &str) -> String {
    let splitted = phrase.split(|c: char| [' ', '-'].contains(&c));

    splitted.filter_map(|w| {
        let word = w.chars().filter(|c| c.is_ascii_alphabetic()).collect::<Vec<char>>();
        match word.first() {
            None => None,
            Some(c) => {
                if c.is_ascii_lowercase() {
                    return Some(c.to_ascii_uppercase().to_string());
                } else {
                    let filtered: String = word.clone().iter().filter(|d| d.is_ascii_uppercase()).collect();
                    if filtered.len() == word.len() {
                        return Some(c.to_string());
                    }
                    Some(filtered)
                }
            }
        }
    }).collect()
}