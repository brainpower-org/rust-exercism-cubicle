use std::iter::FromIterator;
use std::convert::TryFrom;

#[derive(Eq, PartialEq)]
pub enum Bracket {
    Round,
    Curly,
    Square
}

pub enum BracketType {
    Opening(Bracket),
    Closing(Bracket),
}

pub fn brackets_are_balanced(string: &str) -> bool {
    string.chars().collect::<BracketList>().0.into_iter()
        .fold(Ok(vec![]), |stack, b| stack.and_then(|s| process(b, s)))
        .ok()
       .map_or(false, |s| s.is_empty())
}

pub fn process(b: BracketType, mut s: Vec<Bracket>) -> Result<Vec<Bracket>, Vec<Bracket>> {
    match (b, s.last()) {
        (BracketType::Opening(b), _) => { s.push(b); Ok(s) },
        (BracketType::Closing(ref c), Some(d)) if c == d => { s.pop(); Ok(s) },
        _ => { Err(s) }
    }
}

pub fn to_bracket(c: char) -> Option<BracketType> {
    use BracketType::*;
    use Bracket::*;

    match c {
        | '[' => Some(Opening(Square)),
        | '(' => Some(Opening(Round)),
        | '{' => Some(Opening(Curly)),
        | ')' => Some(Closing(Round)),
        | ']' => Some(Closing(Square)),
        | '}' => Some(Closing(Curly)),
        _ => None
    }
}

pub fn get_closing(c: char) -> Option<char> {
    match c {
        '[' => Some(']'),
        '(' => Some(')'),
        '{' => Some('}'),
        _ => None
    }
}


impl TryFrom<char> for BracketType {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
       to_bracket(c).ok_or("No Bracket")
    }
}

struct BracketList(Vec<BracketType>);


impl FromIterator<char> for BracketList {
    

    fn from_iter<I: IntoIterator<Item=char>>(iter: I) -> Self {
        let mut result = vec![];
        for i in iter {
            if let Ok(r) = BracketType::try_from(i) {
                result.push(r);
            }
        }
        BracketList(result)
    }
}