use lazy_static::lazy_static;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::sync::Mutex;

lazy_static! {
    static ref ROBOTS: Mutex<BTreeSet<u32>> = Mutex::new(BTreeSet::new());
}

pub struct Robot {
    number: u32,
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let mut result = Self {
            name: String::new(),
            number: 0,
        };
        result.generate_name();
        result
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    fn generate_name(&mut self) {
        let mut rng = thread_rng();

        let mut first_letter;
        let mut second_letter;
        let mut numbers;

        loop {
            first_letter = rng.gen_range(0..26);
            second_letter = rng.gen_range(0..26);
            numbers = rng.gen_range(0..1000);

            self.number = first_letter * 26 * 1000 + second_letter * 1000 + numbers;

            if ROBOTS.lock().unwrap().insert(self.number) {
                break;
            }
        }

        self.name = format!(
            "{}{}{:03}",
            (first_letter as u8 + 65) as char,
            (second_letter as u8 + 65) as char,
            numbers
        );
    }

    fn delete_name(&self) {
        ROBOTS.lock().unwrap().remove(&self.number);
    }

    pub fn reset_name(&mut self) {
        self.delete_name();
        self.generate_name();
    }
}
