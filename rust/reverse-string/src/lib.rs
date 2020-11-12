use unicode_normalization::UnicodeNormalization;

pub fn reverse(input: &str) -> String {
    let mut s:Vec<String> = vec![];
    for c in input.nfc() {
        s.insert(0, c.to_string());
    }

    return s.join("").nfd().to_string()
}
