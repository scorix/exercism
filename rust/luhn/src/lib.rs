/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }

    if code.chars().any(|c| c.to_digit(10).is_none() && c != ' ') {
        return false;
    }

    let numberics = code.chars().rev().filter_map(|c| c.to_digit(10));

    let res = numberics.enumerate().try_fold(0, |acc, (i, x)| match x {
        0..=4 if i % 2 == 1 => Some(acc + x * 2),
        5..=9 if i % 2 == 1 => Some(acc + x * 2 - 9),
        _ => Some(acc + x),
    });

    match res {
        Some(x) => x % 10 == 0,
        None => false,
    }
}
