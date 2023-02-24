use std::{collections::HashMap, thread, sync::{mpsc, Arc}};
use itertools::Itertools;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = Arc::new(input.join(""));
    let (tx, rx) = mpsc::channel();
        
    let input_len = input.len();
    let character_per_worker = ((input_len + 1) as f32 / worker_count as f32).ceil() as usize;

    for current_worker in 0..worker_count {
        let local = Arc::clone(&input);
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(local[(current_worker*character_per_worker)..((current_worker+1)*character_per_worker).min(input_len)].chars().filter(|&x| x.is_alphabetic()).map(|x| x.to_ascii_lowercase()).counts()).unwrap();
        });
    }
    
    drop(tx);
    
    let mut result = HashMap::new();    

    for hashmap in rx {
        for (char, number) in hashmap {
            *result.entry(char).or_insert(0) += number;
        }
    }

    result
}
