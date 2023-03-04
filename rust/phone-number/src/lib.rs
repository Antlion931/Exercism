pub fn number(user_number: &str) -> Option<String> {
    let mut formated = user_number.chars().filter_map(|x| x.to_digit(10)).collect::<Vec<_>>();

    if formated.len() == 11 && formated[0] == 1 {
        formated.remove(0);
    }

    (formated.len() == 10 && formated[0] > 1 && formated[3] > 1).then_some(formated.iter().map(|x| x.to_string()).collect())
}
