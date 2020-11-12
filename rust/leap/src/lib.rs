pub fn is_leap_year(year: u64) -> bool {
    if year % 400 == 0 {
        return true
    }

    if year % 100 == 0 {
        return false
    }

    return year % 4 == 0
}