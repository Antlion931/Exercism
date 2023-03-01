use rand::prelude::*;

static NUMBER_OF_LETTERS: i32 = 26;
static SMALL_LETTER_OFFSET: i32 = 'a' as i32;

fn encode_with(coder: fn(i32, i32) -> i32, key: &str, s: &str) -> Option<String> {
    s.chars()
        .zip(key.chars().cycle())
        .map(|(a, b)| {
            if a.is_ascii_lowercase() && b.is_ascii_lowercase() {
                Some((coder(a as i32, b as i32).rem_euclid(NUMBER_OF_LETTERS) + SMALL_LETTER_OFFSET) as u8 as char)
            } else {
                None
            }
        })
        .collect::<Option<String>>()
        .filter(|text| !text.is_empty())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    encode_with(|a, b| a + b - 2*SMALL_LETTER_OFFSET, key, s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    encode_with(|a, b| a - b, key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();

    let key = (0..200).map(|_| rng.gen_range('a'..='z')).collect::<String>();
    let encoded = encode(&key, &s.to_lowercase()).expect("s should be correct");
    (key, encoded)
}
