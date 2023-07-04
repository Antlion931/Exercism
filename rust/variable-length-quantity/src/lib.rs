#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(numbers: &[u32]) -> Vec<u8> {
    let f = |number: &u32| {
        let mut number = *number;
        let mut result = Vec::new();

        while number > 0 || result.is_empty() {
            let mut tail = (number % 0b1000_0000) as u8;
            number >>= 7;

            if !result.is_empty() {
                tail |= 0b1000_0000;
            }

            result.push(tail);
        }

        result.into_iter().rev()
    };

    numbers.iter().flat_map(f).collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut number: u32 = 0;

    if bytes.last().filter(|&&x| x >= 0b1000_0000).is_some() {
        return Err(Error::IncompleteNumber);
    }

    for byte in bytes {
        let decoded = (byte % 0b1000_0000) as u32;

        if number.leading_zeros() < 7 {
            return Err(Error::Overflow);
        }

        number = (number << 7) + decoded;

        if *byte < 0b1000_0000 {
            result.push(number);
            number = 0;
        }
    }

    Ok(result)
}
