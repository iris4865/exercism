use std::{collections::HashSet, ops::Deref};

#[cfg(test)]
mod tests;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let answer = sorted_lowercase_string(word);

    possible_anagrams
        .into_iter()
        .filter(|w| w.to_lowercase() != word.to_lowercase())
        .filter(|w| sorted_lowercase_string(*w) == answer)
        .map(Deref::deref)
        .collect()
}

fn sorted_lowercase_string(word: &str) -> String {
    let mut lowercase_word: Vec<_> = word.to_lowercase().chars().collect();
    lowercase_word.sort();
    lowercase_word.into_iter().collect()
}
