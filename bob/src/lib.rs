fn is_all_caps(message: &str) -> bool {
    let mut has_letters = false;
    for c in message.chars() {
        if c.is_alphabetic() {
            if c.to_lowercase().to_string() == c.to_string() {
                return false;
            }
            has_letters = true;
        }
    }
    return has_letters;
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        x if x.len() == 0 => "Fine. Be that way!",
        x if x.ends_with("?") => {
            if is_all_caps(x) {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        x if is_all_caps(x) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
