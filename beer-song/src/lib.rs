pub fn verse(n: i32) -> String {
    match n {
        n if n > 2 => format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n\
             Take one down and pass it around, {} bottles of beer on the wall.\n",
            n,
            n,
            n - 1
        ),
        n if n == 2 => String::from(
            "2 bottles of beer on the wall, 2 bottles of beer.\n\
             Take one down and pass it around, 1 bottle of beer on the wall.\n",
        ),
        n if n == 1 => String::from(
            "1 bottle of beer on the wall, 1 bottle of beer.\n\
             Take it down and pass it around, no more bottles of beer on the wall.\n",
        ),
        _ => String::from(
            "No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
    }
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev()
        .map(|i| verse(i))
        .collect::<Vec<String>>()
        .join("\n")
}
