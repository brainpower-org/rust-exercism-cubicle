pub fn verse(n: i32) -> String {
    match dbg!(n) {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        n @ 3...99 => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {m} bottles of beer on the wall.\n", n=n, m=(n-1)),
        _ => unreachable!()
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
