/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let chars = code.chars().filter(|c| !c.is_whitespace()); 
    
    if chars.clone().count() < 2 || chars.clone().any(|c| !c.is_digit(10)) { return false;}
    
    chars.rev().enumerate().map(|(i, c)| {
        let x = (i as u32 % 2 + 1) * c.to_digit(10).unwrap();

        if x > 9 { x - 9 }
        else { x }
        }).sum::<u32>() % 10 == 0
}
