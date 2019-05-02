/// # Solution 1
/// 
/// ```rust
/// pub fn build_proverb(list: &[&str]) -> String {
///     list.iter()
///         .enumerate()
///         .filter_map(|(i, _)| verse(list, i))
///         .collect::<Vec<String>>()
///         .join("\n")
/// }
///
/// pub fn verse(list: &[&str], index: usize) -> Option<String> {
///     match (list.get(index), list.get(index + 1)) {
///         (Some(w1), Some(w2)) => Some(format!("For want of a {w1} the {w2} was lost.", w1 = w1, w2 = w2)),
///         (Some(_), None) => Some(format!("And all for the want of a {w}.", w = list[0])),
///         (_, _) => None
///     }
/// }
/// 
/// # fn test_two_pieces() {
/// #     let input = vec!["nail", "shoe"];
/// #     let expected = vec![
/// #         "For want of a nail the shoe was lost.",
/// #         "And all for the want of a nail.",
/// #     ]
/// #     .join("\n");
/// #     assert_eq!(build_proverb(&input), expected);
/// # }
/// #
/// # fn test_three_pieces() {
/// #     let input = vec!["nail", "shoe", "horse"];
/// #     let expected = vec![
/// #         "For want of a nail the shoe was lost.",
/// #         "For want of a shoe the horse was lost.",
/// #         "And all for the want of a nail.",
/// #     ]
/// #     .join("\n");
/// #     assert_eq!(build_proverb(&input), expected);
/// # }
/// #
/// # fn test_one_piece() {
/// #     let input = vec!["nail"];
/// #     let expected = String::from("And all for the want of a nail.");
/// #     assert_eq!(build_proverb(&input), expected);
/// # }
/// #
/// # fn test_zero_pieces() {
/// #     let input: Vec<&str> = vec![];
/// #     let expected = String::new();
/// #     assert_eq!(build_proverb(&input), expected);
/// # }
/// #
/// # fn test_full() {
/// #     let input = vec![
/// #         "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
/// #     ];
/// #     let expected = vec![
/// #         "For want of a nail the shoe was lost.",
/// #         "For want of a shoe the horse was lost.",
/// #         "For want of a horse the rider was lost.",
/// #         "For want of a rider the message was lost.",
/// #         "For want of a message the battle was lost.",
/// #         "For want of a battle the kingdom was lost.",
/// #         "And all for the want of a nail.",
/// #     ]
/// #     .join("\n");
/// #     assert_eq!(build_proverb(&input), expected);
/// # }
/// # 
/// # fn test_three_pieces_modernized() {
/// #     let input = vec!["pin", "gun", "soldier", "battle"];
/// #     let expected = vec![
/// #         "For want of a pin the gun was lost.",
/// #         "For want of a gun the soldier was lost.",
/// #         "For want of a soldier the battle was lost.",
/// #         "And all for the want of a pin.",
/// #     ]
/// #     .join("\n");
/// #     assert_eq!(build_proverb(&input), expected);
/// # }
/// #
/// # test_two_pieces();
/// # test_three_pieces();
/// # test_one_piece();
/// # test_zero_pieces();
/// # test_full();
/// # test_three_pieces_modernized();
/// #
/// ```

pub fn build_proverb(list: &[&str]) -> String {
    let words = list
        .iter()
        .take(1)
        .map(|w| *w)
        .collect::<Vec<&str>>();
    
    list
        .windows(2)
        .chain(std::iter::once(words.as_slice()))
        .map(|win| verse(win))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn verse(words: &[&str]) -> String {
    match words {
        [w1, w2] => format!("For want of a {w1} the {w2} was lost.", w1 = w1, w2 = w2),
        [w] => format!("And all for the want of a {w}.", w = w),
        [] => String::from(""),
        _ => unreachable!("Encountered verse word list with invalid length")
    }
}