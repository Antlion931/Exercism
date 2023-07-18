pub fn encrypt(input: &str) -> String {
    let normalized: Vec<_> = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    if normalized.is_empty() {
        return String::new();
    }

    let x = (normalized.len() as f32).sqrt().floor() as usize;

    let c = if normalized.len() <= x * x { x } else { x + 1 };

    let mut result = String::with_capacity(normalized.len());

    for i in 0..c {
        for chunk in normalized.chunks(c) {
            let letter = chunk.get(i).copied().unwrap_or(' ');

            result.push(letter);
        }

        if i + 1 < c {
            result.push(' ');
        }
    }

    result
}
