pub fn is_armstrong_number(num: u32) -> bool {
    let num_s = num.to_string();
    let len = num_s.len() as u32;
    let mut buffor = num;

    for x in num_s.chars().map(|c| c.to_digit(10).unwrap().pow(len)) {
        if buffor >= x {
            buffor -= x;
        } else {
            return false;
        }
    }
    buffor == 0
}
