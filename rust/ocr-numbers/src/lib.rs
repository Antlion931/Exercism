// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<_> = input.lines().collect();

    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }

    let mut result: Vec<String> = Vec::with_capacity(lines.len() / 4);

    for digit_row in lines.chunks(4) {
        let first_row_lenght = digit_row[0].len();
        if first_row_lenght % 3 != 0 {
            return Err(Error::InvalidColumnCount(first_row_lenght));
        }

        let mut digits = vec![String::with_capacity(9); first_row_lenght / 3];

        for row in digit_row.iter().take(3) {
            if row.len() != first_row_lenght {
                return Err(Error::InvalidColumnCount(row.len()));
            }
            let mut iterator = row.chars();

            for d in digits.iter_mut() {
                for _ in 0..3 {
                    d.push(
                        iterator
                            .next()
                            .expect("Multiple of 3 and same as first row"),
                    );
                }
            }
        }

        result.push(
            digits
                .into_iter()
                .map(|d| match d.as_str() {
                    " _ | ||_|" => '0',
                    "     |  |" => '1',
                    " _  _||_ " => '2',
                    " _  _| _|" => '3',
                    "   |_|  |" => '4',
                    " _ |_  _|" => '5',
                    " _ |_ |_|" => '6',
                    " _   |  |" => '7',
                    " _ |_||_|" => '8',
                    " _ |_| _|" => '9',
                    _ => '?',
                })
                .collect(),
        );
    }

    Ok(result.join(","))
}
