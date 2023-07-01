pub fn number(user_number: &str) -> Option<String> {
    let mut formated: Vec<_> = user_number.chars().filter_map(|x| x.to_digit(10)).collect();

    if formated.len() == 11 && formated[0] == 1 {
        formated.remove(0);
    }

    (formated.len() == 10 && formated[0] > 1 && formated[3] > 1).then(|| {
        formated
            .into_iter()
            .fold(0, |acc, x| acc * 10 + x)
            .to_string()
    })
}
