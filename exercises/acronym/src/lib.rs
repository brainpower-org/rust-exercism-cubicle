pub fn abbreviate(phrase: &str) -> String {
    let splitted = phrase.split(|c: char| [' ', '-'].contains(&c));

    splitted.filter_map(|w| {
        let mut word = w.chars().filter(|c| c.is_ascii_alphanumeric());
        match word.next() {
        None => None,
        Some(c) => {
            if c.is_ascii_lowercase() {
                return Some(c.to_ascii_uppercase().to_string());
            } else {
                let filtered: String = word.clone().filter(|d| d.is_ascii_uppercase()).collect();
                if filtered.len() == word.count() {
                    return Some(c.to_string());
                }
                Some(filtered)
            }
        }
        }
    }).collect()
}