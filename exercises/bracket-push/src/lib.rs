pub fn is_opening (c: char) -> bool {
    match c { 
        '[' | '(' | '{' => true,
        _ => false
    }
}

pub fn is_bracket (c: char) -> bool {
    match c { 
        '[' | ']' | '(' | ')' | '{' | '}' => true,
        _ => false
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .filter(|c| is_bracket(*c))
        .fold(vec![], |mut acc, c| {
            match (c, acc.last()) {
               ('[', _) | ('(', _) | ('{', _) => { acc.push(c); acc },
               (']', Some('[')) | ('}', Some('{')) | (')', Some('(')) => {
                   acc.pop();
                   acc
               },
               (']', _) | ('}', _) | (')', _) => {
                   acc.push(c); 
                   acc
               },
               _ => acc
            }
        })
        .len() == 0
}
