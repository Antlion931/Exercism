pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    match items {
        [] => 0,
        [item, ..] if item.weight > max_weight => maximum_value(max_weight, &items[1..]),
        [item, ..] => {
            let take = maximum_value(max_weight - item.weight, &items[1..]) + item.value;
            let not_take = maximum_value(max_weight, &items[1..]);

            take.max(not_take)
        }
    }
}
