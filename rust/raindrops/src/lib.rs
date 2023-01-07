pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    let factors_and_sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    for (f, s) in factors_and_sounds {
        if n % f == 0 {
            result += s;
        }
    }

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
