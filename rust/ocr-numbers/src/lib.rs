// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let rows: Vec<&str> = input.split("\n").collect();

    if rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(3));
    }

    if rows.iter().any(|row| row.len() % 3 > 0) {
        return Err(Error::InvalidColumnCount(4));
    }

    // calculate chars and initialize vectors
    let char_size = 12;
    let ocr_chars_len = rows.iter().map(|row| row.len()).sum::<usize>() / char_size;
    let chars_per_row = rows[0].len() / 3;
    let mut decomposed_chars: Vec<Vec<String>> =
        vec![vec!["".to_string(); chars_per_row]; ocr_chars_len / chars_per_row];

    for lineno in 0..rows.len() {
        if (lineno + 1) % 4 == 0 {
            continue;
        }
        for i in 0..chars_per_row {
            let (start, end) = (i * 3, i * 3 + 3);
            decomposed_chars[lineno / 4][i].push_str(&rows[lineno][start..end]);
        }
    }

    Ok(decomposed_chars
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| recognize(c.as_str()))
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join(","))
}

fn recognize(s: &str) -> &str {
    match s {
        " _ | ||_|" => "0",
        "     |  |" => "1",
        " _  _||_ " => "2",
        "   |_|  |" => "4",
        " _  _| _|" => "3",
        " _ |_  _|" => "5",
        " _ |_ |_|" => "6",
        " _   |  |" => "7",
        " _ |_||_|" => "8",
        " _ |_| _|" => "9",
        "" => "",
        _ => "?",
    }
}
