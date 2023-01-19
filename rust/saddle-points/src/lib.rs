pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.len() == 0 || input[0].len() == 0 {
        return Vec::new();
    }

    let columns_size = input[0].len();
    let rows_size = input.len();

    let mut columns_mins = Vec::new();
    for c in 0..columns_size {
        let mut min = input[0][c];
        for r in 1..rows_size {
            if input[r][c] < min {
                min = input[r][c];
            }
        }
        columns_mins.push(min);
    }
    
    let rows_maxs = input.iter().map(|v| *v.iter().max().expect("Already checked that it won't be empty")).collect::<Vec<_>>();
    
    let mut result = Vec::new(); 
    for r in 0..rows_size {
        for c in 0..columns_size {
            if input[r][c] == columns_mins[c] && input[r][c] == rows_maxs[r] {
                result.push((r, c));
            }
        }
    }
    result
}

