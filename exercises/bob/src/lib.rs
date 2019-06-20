pub fn shouting(message: &str) -> bool {
    let letters = message.chars()
        .filter(|c: &char| c.is_alphabetic())
        .collect::<String>();

    letters.len() > 0 && letters.to_uppercase() == letters
}

pub fn shouting_peek(message: &str) -> bool {
    let mut letters = message.chars()
        .filter(|c: &char| c.is_alphabetic());
        
    letters.peek().is_some() && letters.all(|c: char| c.is_uppercase())
}

pub fn shouting_imperative(message: &str) -> bool {
    let result = true;
    for c in message.it {
        c
    }
    result
}

pub fn reply(message: &str) -> &str { 
    let message = message.trim();
    let shouts = shouting_peek(message);
    let asks = message.ends_with("?");
    let empty = message.len() == 0;

    if shouts && asks {
        "Calm down, I know what I'm doing!"
    } else if empty {
        "Fine. Be that way!"
    } else if asks {
        "Sure."
    } else if shouts {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}