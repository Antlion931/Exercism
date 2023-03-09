use num_integer::Integer;
use itertools::Itertools;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn validate_coprime(a: i32) -> Result<(), AffineCipherError> {
    if a.gcd(&26) == 1 { Ok(()) }
    else { Err(AffineCipherError::NotCoprime(a)) }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    validate_coprime(a)?;
    Ok(plaintext.chars()
        .filter(char::is_ascii_alphanumeric)
        .map( |char| char.to_ascii_lowercase() )
        .map(|char| {
            if char.is_numeric() {
                char
            } else {
                let x = (char as u8 - 'a' as u8) as i32;
                (((a * x + b) % 26) as u8 + 'a' as u8) as char 
            }
        })
        .chunks(5)
        .into_iter()
        .map(|x| x.collect::<String>() )
        .collect_vec()
        .join(" "))
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    validate_coprime(a)?;
    let a = (1..26).find(|n| n*a % 26 == 1).unwrap();
    Ok(ciphertext.chars()
       .filter(char::is_ascii_alphanumeric)
       .map(|char| char.to_ascii_lowercase())
       .map(|char| {
            if char.is_numeric() {
                char
            } else {
                let x = (char as u8 - 'a' as u8) as i32;
                println!("{a}");
                (( ((x as i32 - b) * a).rem_euclid(26) ) as u8 + 'a' as u8) as char
            }
        })
       .collect::<String>())
}
