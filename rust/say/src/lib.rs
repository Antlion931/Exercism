const SMALL_NUMBERS: [&str; 20] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];

const TENS_NUMBERS: [&str; 8] = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

const LARGE_NUMBERS: [&str; 6] = ["thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

fn encode_2_digits(n: u64) -> String {
    if n < 20 {
        SMALL_NUMBERS[n as usize].into()
    } else if n < 100 {
        let (tens, ones) = (n / 10 - 2, n % 10); // no panic because n >= 20

        if ones == 0 {
            TENS_NUMBERS[tens as usize].to_string()
        } else {
            TENS_NUMBERS[tens as usize].to_string() + "-" + SMALL_NUMBERS[ones as usize]
        }
    } else {
        panic!("more then 2 digits: {}", n)
    }
}

fn encode_3_digits(n: u64) -> String {
    if n < 1000 {
        let (hundreds, rest) = (n / 100, n % 100);

        let mut result = if hundreds != 0 {
            encode_2_digits(hundreds) + " hundred" 
        } else {
            String::new()
        };

        if rest != 0 {
            if result.is_empty() {
                result = encode_2_digits(rest);
            } else {
                result = result + " " + encode_2_digits(rest).as_str();
            }
        }

        result
    } else {
        panic!("more then 2 digits: {}", n)
    }
}

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return SMALL_NUMBERS[0].to_string();
    }

    let mut index = 0;
    let mut result = String::new();
    while n > 0 {
        let number = n % 1000;
        n /= 1000;

        if number != 0 {
            let new = if index > 0 {
                encode_3_digits(number) + " " + LARGE_NUMBERS[index - 1]
            } else {
                encode_3_digits(number)
            };


            if result.is_empty() {
                result = new
            } else {
                result = new + " " + result.as_str();
            }
        }
        index += 1;
    }
    
    result
}
