use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PokerHand {
    rank: Vec<i32>,
    cards: Vec<i32>,
}

fn make_cards(y: &Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let mut map = HashMap::new();
    for s in y {
        *map.entry(
        match s.split_at(s.len() - 1).0.parse::<i32>() {
            Ok(v) => v,
            Err(_) => "JQKA".find(s.split_at(s.len() - 1).0).unwrap() as i32 + 11,
        } ).or_insert(0) += 1;
    }
    let mut result = vec![];
    let mut values: Vec<_> = map.values().collect();
    values.sort();
    values.reverse();
    values.dedup();

    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();
    keys.reverse();

    for v in values {
        for k in keys.iter() {
            if &map[k] == v {
                result.push(**k);
            }
        }
    }

    let mut values: Vec<i32> = map.values().map(|x| *x).collect();
    values.sort();
    values.reverse();

    (result, values)
}

impl PokerHand {
    fn new(hand: &str) -> Self {
        let y = hand.split_whitespace().into_iter().collect::<Vec<_>>();
        let (mut cards, test) = make_cards(&y);

        match test[..]{
            [1, 1, 1, 1, 1] => {
                let is_flush = {
                    let first = y[0].chars().last().unwrap();
        
                    y.iter().map(|s| s.chars().last().unwrap()).filter(|c| c == &first).count() == 5
                };
        
                let mut is_straight = *cards.first().unwrap() as i8 - *cards.last().unwrap() as i8 == 4;
                if cards ==  vec![14, 5, 4, 3, 2] {
                    is_straight = true;
                    cards.rotate_left(1);
                };
        
                match (is_flush, is_straight) {
                    (true, true) => PokerHand {rank: vec![4, 1, 1], cards},
                    (true, false) => PokerHand {rank: vec![3, 1, 1, 1, 1], cards},
                    (false, true) => PokerHand {rank: vec![3, 1, 1, 1], cards},
                    _ => PokerHand {rank: vec![1, 1, 1, 1, 1], cards},
                }
            }
            _ => PokerHand { rank: test, cards}
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut poker_hands: BinaryHeap<_> = hands.into_iter()
        .map(|hand| (PokerHand::new(hand), hand))
        .collect();

    let max = poker_hands.peek().unwrap().0.clone();

    let mut result = vec![];
    
    while let Some((value, s)) = poker_hands.pop(){
        if value < max {
            break;
        }
        result.push(*s);
    }

    result
}
