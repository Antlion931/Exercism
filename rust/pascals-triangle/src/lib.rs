pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        if row_count == 0 {
            return Self { rows: Vec::new() };
        }

        let row_count = row_count as usize;

        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count);
        rows.push(vec![1]);

        for i in 1..row_count {
            let mut row = Vec::with_capacity(i);
            row.push(1);
            for k in 1..i {
                row.push(rows[i - 1][k] + rows[i - 1][k - 1])
            }
            row.push(1);
            rows.push(row);
        }

        Self { rows }
    }

    /*    pub fn new_2(row_count: u32) -> Self {
            let rows: Vec<Vec<u32>> = (0..row_count).map(|n| {
                let mut vec = Vec::with_capacity(n as usize);
                let mut last = 1;
                vec.push(last);
                for k in 0..n {
                    last *= n - k;
                    last /= k + 1;
                    vec.push(last);
                }
                vec
            }).collect();

            Self { rows }
        }
    */
    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
