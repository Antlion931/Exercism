use itertools::Itertools;

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];
pub fn score(mut dice: Dice, category: Category) -> u8 {
    dice.sort();
    match category {
        Category::Ones => dice.into_iter().filter(|x| *x == 1).sum(),
        Category::Twos => dice.into_iter().filter(|x| *x == 2).sum(),
        Category::Threes => dice.into_iter().filter(|x| *x == 3).sum(),
        Category::Fours => dice.into_iter().filter(|x| *x == 4).sum(),
        Category::Fives => dice.into_iter().filter(|x| *x == 5).sum(),
        Category::Sixes => dice.into_iter().filter(|x| *x == 6).sum(),
        Category::LittleStraight if dice == [1, 2, 3, 4, 5] => 30,
        Category::BigStraight if dice == [2, 3, 4, 5, 6] => 30,
        Category::FullHouse if dice.iter().counts().values().all(|x| *x == 3 || *x == 2) => dice.into_iter().sum(),
        Category::FourOfAKind if dice[0] == dice[3] => dice.into_iter().rev().skip(1).sum(),
        Category::FourOfAKind if dice[1] == dice[4] => dice.into_iter().skip(1).sum(),
        Category::Choice => dice.into_iter().sum(),
        Category::Yacht if dice.iter().all_equal() => 50,
        _ => 0,
    }
}
