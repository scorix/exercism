use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .cloned()
        .filter(|a| anagram_for(word, a))
        .collect()
}

fn anagram_for(word: &str, anagram: &str) -> bool {
    if word.len() != anagram.len() {
        return false;
    }

    let mut wcs = word
        .chars()
        .map(|a| a.to_lowercase().to_string())
        .collect::<Vec<_>>();
    let mut ncs = anagram
        .chars()
        .map(|a| a.to_lowercase().to_string())
        .collect::<Vec<_>>();

    if wcs == ncs {
        return false;
    }

    wcs.sort();
    ncs.sort();

    wcs == ncs
}
