const VOWELS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];
// sorted so that patterns will not colidate when iterating from begin to end
const CONSTANTS: [&str; 80] = [
    "zz", "ze", "z", "y", "x", "wr", "wh", "w", "ve", "v", "u", "tu", "tt", "ti", "thr", "th",
    "te", "tch", "t", "st", "ss", "si", "sh", "se", "sci", "sch", "sc", "s", "rr", "rh", "r", "qu",
    "q", "ps", "pp", "pn", "ph", "p", "o", "nn", "ngue", "ng", "n", "mn", "mm", "mb", "m", "lm",
    "ll", "lk", "lf", "l", "kn", "k", "j", "i", "h", "gue", "gu", "gn", "gh", "gg", "ge", "g",
    "ft", "ff", "f", "ed", "di", "dge", "dd", "d", "ck", "ci", "ch", "ce", "cc", "c", "bb", "b",
];

pub fn translate(input: &str) -> String {
    let mut results = Vec::new();

    'outer: for w in input.split_whitespace() {
        for v in VOWELS {
            if w.starts_with(v) {
                results.push(w.to_string() + "ay");
                continue 'outer;
            }
        }

        for c in CONSTANTS {
            if let Some(stripped) = w.strip_prefix(c) {
                let from = if stripped.starts_with("qu") {
                    c.len() + 2
                } else {
                    c.len()
                };

                results.push(w[from..].to_string() + &w[..from] + "ay");

                continue 'outer;
            }
        }
    }

    results.join(" ")
}
