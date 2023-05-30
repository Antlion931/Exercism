pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut result = vec![vec![0; size]; size];

    let mut number = 1;

    for i in 0..(size + 1) / 2 {
        // GO RIGHT
        for x in i..(size - i) {
            result[i][x] = number;
            number += 1;
        }

        // GO DOWN
        for x in i + 1..(size - i) {
            result[x][size - i - 1] = number;
            number += 1;
        }

        // GO LEFT
        for x in (i..(size - i - 1)).rev() {
            result[size - i - 1][x] = number;
            number += 1;
        }

        // GO UP
        for x in (i + 1..(size - i - 1)).rev() {
            result[x][i] = number;
            number += 1;
        }
    }

    result
}
