use std::collections::{HashMap, HashSet};

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = HashMap::new();
    brackets.insert('{', '}');
    brackets.insert('(', ')');
    brackets.insert('[', ']');

    let left_brackets: HashSet<char> = vec!['{', '(', '['].into_iter().collect();
    let right_brackets: HashSet<char> = vec!['}', ')', ']'].into_iter().collect();

    let mut stack = vec![];

    for letter in string.chars() {
        match letter {
            c if left_brackets.contains(&c) => stack.push(c),
            c if right_brackets.contains(&c) => match stack.pop() {
                Some(pair) => {
                    if *brackets.get(&pair).unwrap() != c {
                        return false;
                    }
                }
                None => return false,
            },
            _ => (),
        }
    }

    stack.len() == 0
}
