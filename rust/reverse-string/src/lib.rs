use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut chars: Vec<_> = UnicodeSegmentation::graphemes(input, true).collect();
    chars.reverse();
    chars.into_iter().collect()
}
