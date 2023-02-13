pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let width = minefield.get(0).map_or(0, |str| str.len());
    let height = minefield.len();

    let mut mine_neighbors_conts = (0..height).map(|_| (0..width).map(|_| 0).collect::<Vec<_>>() ).collect::<Vec<_>>();

    for x in 0..width {
        for y in 0..height {
            if minefield[y].chars().nth(x).unwrap() == '*' {
                let x_begin = if x == 0 {0} else {x - 1};
                let x_end = (x + 1).min(width - 1);

                let y_begin = if y ==0 {0} else {y - 1};
                let y_end = (y + 1).min(height - 1);

                for xx in x_begin..=x_end {
                    for yy in y_begin..=y_end {
                        mine_neighbors_conts[yy][xx] += 1;
                    }
                }
            }
        }
    }

    let mut result = Vec::new();

    for y in 0..height {
        result.push(
            minefield[y].chars().enumerate().map(|(x, c)| {
            if c == '*' || mine_neighbors_conts[y][x] == 0 {
                return c;
            }

            mine_neighbors_conts[y][x].to_string().chars().nth(0).unwrap()
        }).collect());
        
    }

    result
}
