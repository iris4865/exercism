use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

pub fn reverse(input: &str) -> String {
    // https://github.com/unicode-rs/unicode-segmentation/blob/9d79ee2acbe159d3557918ca1a5290995eef9a7e/src/grapheme.rs#L106
    input.graphemes(true).rev().collect()
}
