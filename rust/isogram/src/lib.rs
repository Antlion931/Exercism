pub fn check(candidate: &str) -> bool {
    let candidate = candidate.to_lowercase();

    candidate.chars()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .all(|(n, c)| candidate.rfind(c) == Some(n))
}
