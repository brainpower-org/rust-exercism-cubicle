/// ```
/// pub fn is_bracket (c: char) -> bool {
///     match c {
///         '[' | ']' | '(' | ')' | '{' | '}' => true,
///         _ => false
///     }
/// }
///
/// pub fn brackets_are_balanced(string: &str) -> bool {
///     string
///         .chars()
///         .filter(|c| is_bracket(*c))
///         .fold(vec![], |mut acc, c| {
///             match (c, acc.last()) {
///                ('[', _) | ('(', _) | ('{', _) => { acc.push(c); acc },
///                (']', Some('[')) | ('}', Some('{')) | (')', Some('(')) => {
///                    acc.pop();
///                    acc
///                },
///                (']', _) | ('}', _) | (')', _) => {
///                    acc.push(c);
///                    acc
///                },
///                _ => acc
///             }
///         })
///         .len() == 0
/// }
/// ```
///
///

pub fn is_bracket(c: &char) -> bool {
    match c {
        '[' | '(' | '{' | ']' | ')' | '}' => true,
        _ => false,
    }
}

pub fn is_opening(c: char) -> Option<char> {
    match c {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        _ => None,
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .filter(is_bracket)
        .fold(Some(&mut vec![]), |mut stack, c| {
            match (c, stack.as_mut()) {
                ('[', Some(s)) => s.push(']'),
                ('(', Some(s)) => s.push(')'),
                ('{', Some(s)) => s.push('}'),
                (x, Some(s)) => {
                    if s.last() == Some(&x) {
                        s.pop();
                    } else {
                        stack = None;
                    }
                }
                (_, _) => (),
            };
            stack
        })
        .map_or(false, |s| s.iter().peekable().peek().is_none())
}
