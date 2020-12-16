use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let mut word_chars = word.chars().collect::<Vec<char>>();
    word_chars.sort_unstable();

    possible_anagrams
        .iter()
        .cloned()
        .filter(|a| anagram_for(&word, &word_chars, a))
        .collect()
}

fn anagram_for(word: &str, word_chars: &Vec<char>, anagram: &str) -> bool {
    let anagram = anagram.to_lowercase();

    if word == anagram || word.len() != anagram.len() {
        return false;
    }

    let mut anagram_chars = anagram.chars().collect::<Vec<char>>();
    anagram_chars.sort_unstable();

    word_chars == &anagram_chars
}
