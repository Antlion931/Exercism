fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn abbreviate(phrase: &str) -> String {
    let x = phrase.replace("-", " ")
        .replace(":", "")
        .split_whitespace()
        .map(|s| {
            if s.chars().all(|c| c.is_uppercase()) {
                s.chars().nth(0).expect("Should have at least one char").to_string()
            } else {
                capitalize_first_letter(s).chars().filter(|c| c.is_uppercase()).collect::<String>()      
            }
        })
        .collect();

    println!("{x}");
    x
}
