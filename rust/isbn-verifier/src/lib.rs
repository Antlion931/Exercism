pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.chars()
                .filter(|char| *char != '-')
                .collect::<Vec<_>>();

    let (checksum, digits) = isbn.split_last().unwrap_or((&'X', &[]));

    if digits.len() != 9 || !digits.iter().all(char::is_ascii_digit) || (!checksum.is_digit(10) && *checksum != 'X'){
        return false;
    }

    let reminder;
    let sum: u32 = digits.iter()
        .enumerate()
        .map(|(in_order, digit)| digit.to_digit(10).expect("Already checked") * (10 - in_order as u32))
        .sum();

    if let Some(number) = checksum.to_digit(10) {
        reminder = 11 - number;
    } else { // checksum is X, and 11 - 10 = 1
        reminder = 1;
    }

    sum % 11 == reminder
}
