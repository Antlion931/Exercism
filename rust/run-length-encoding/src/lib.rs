use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .dedup_with_count()
        .map(|(n, c)| {
            if n == 1 {
                c.to_string()
            } else {
                let mut result = n.to_string();
                result.push(c);
                result
            }
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut times = 0;

    for c in source.chars() {
        if c.is_ascii_digit() {
            times *= 10;
            times += c.to_digit(10).expect("is ascii digit");
        } else {
            for _ in 0..times.max(1) {
                result.push(c);
            }
            times = 0;
        }
    }

    result
}
