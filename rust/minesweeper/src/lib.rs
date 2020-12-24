pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mines = list_mines(minefield);
    let annotated = count_mines(minefield, mines);
    format_annotate(annotated)
}

fn format_annotate(annotated: Vec<Vec<i8>>) -> Vec<String> {
    annotated
        .iter()
        .map(|row| {
            row.iter()
                .map(|&ele| match ele {
                    x if x > 0 => x.to_string(),
                    x if x == 0 => " ".to_string(),
                    _ => "*".to_string(),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}

fn list_mines(minefield: &[&str]) -> Vec<(u8, u8)> {
    let mut mines = vec![];
    for row in 0..minefield.len() {
        for col in 0..minefield[0].len() {
            match minefield[row].chars().nth(col) {
                Some('*') => mines.push((row as u8, col as u8)),
                _ => continue,
            }
        }
    }

    mines
}

fn list_around_points(point: (i8, i8), rows: i8, cols: i8) -> Vec<(i8, i8)> {
    let (x, y) = point;
    let points: Vec<(i8, i8)> = vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];
    points
        .into_iter()
        .filter(|&(px, py)| px >= 0 && px < rows && py >= 0 && py < cols)
        .collect()
}

fn count_mines(minefield: &[&str], mines: Vec<(u8, u8)>) -> Vec<Vec<i8>> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }

    let cols = minefield[0].len();

    let mut annotated: Vec<Vec<i8>> = vec![vec![0; cols]; rows];

    for (mine_x, mine_y) in mines {
        annotated[mine_x as usize][mine_y as usize] = -9;
        for (x, y) in list_around_points((mine_x as i8, mine_y as i8), rows as i8, cols as i8) {
            annotated[x as usize][y as usize] += 1;
        }
    }

    annotated
}
