#[derive(Clone, Debug)]
enum FieldType {
    Bomb,
    Number(u8),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let width = minefield.first().map_or(0, |str| str.len());
    let height = minefield.len();

    let mut minefield_types = vec![vec![FieldType::Number(0); width]; height];

    for (y, row) in minefield.iter().enumerate() {
        for (x, _) in row.chars().enumerate().filter(|(_, c)| *c == '*') {
            minefield_types[y][x] = FieldType::Bomb;
            let x_begin = x.saturating_sub(1);
            let x_end = (x + 1).min(width - 1);

            let y_begin = y.saturating_sub(1);
            let y_end = (y + 1).min(height - 1);

            for xx in x_begin..=x_end {
                for yy in y_begin..=y_end {
                    if let FieldType::Number(ref mut x) = minefield_types[yy][xx] {
                        *x += 1;
                    }
                }
            }
        }
    }

    let mut result = Vec::with_capacity(height);

    for row in minefield_types {
        result.push(
            row.iter()
                .map(|x| match x {
                    FieldType::Bomb => '*',
                    FieldType::Number(0) => ' ',
                    FieldType::Number(n) => {
                        char::from_digit(*n as u32, 10).expect("It will be max 8")
                    }
                })
                .collect(),
        );
    }

    result
}
