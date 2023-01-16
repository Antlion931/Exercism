pub fn build_proverb(list: &[&str]) -> String {
    if list.len() < 1 {
        return String::new()
    }

    list.windows(2)
        .map(|words| format!("For want of a {} the {} was lost.\n", words[0], words[1]))
        .rfold(format!("And all for the want of a {}.", list[0]), |acc, line| line + &acc)
}
