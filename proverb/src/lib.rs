pub fn build_proverb(list: &[&str]) -> String {
    let list_length = list.len();
    list.iter()
        .enumerate()
        .map(|(i, word)| {
            if i < list_length - 1 {
                format!("For want of a {} the {} was lost.", word, list[i + 1])
            } else {
                format!("And all for the want of a {}.", list[0])
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}
