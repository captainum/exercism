use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Category {
    Ones = 1,
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

fn calculate_number_of(value: u8, dice: &Dice) -> u8 {
    value * dice.iter().filter(|&x| *x == value).count() as u8
}

fn calculate_nth_of_one(at_least: u8, dice: &Dice) -> u8 {
    let mut hm = HashMap::<u8, u8>::new();
    for &value in dice {
        *hm.entry(value).or_default() += 1;
    }

    let mut keys = hm.keys();

    if keys.len() == 2 {
        let key1 = keys.next().unwrap();
        let key2 = keys.next().unwrap();

        let value1 = *hm.get(key1).unwrap();
        let value2 = *hm.get(key2).unwrap();

        if value1 == at_least || value1 == 5 - at_least {
            if at_least == 3 {
                *key1 * value1 + *key2 * value2
            } else if at_least == 4 && value1 == 4 {
                *key1 * value1
            } else {
                *key2 * value2
            }
        } else { 0 }
    } else if keys.len() == 1 && at_least == 4 {
        keys.nth(0).unwrap() * at_least
    }
    else { 0 }
}

fn calculate_straight(start_from: u8, dice: &Dice) -> u8 {
    match (start_from..5+start_from).all(
        |val| {
            dice.contains(&val)
        }
    ) {
        true => 30,
        false => 0,
    }
}

pub fn score(dice: Dice, category: Category) -> u8 {
    println!("{}", category.clone() as u8);
    match category {
        Category::Ones | Category::Twos | Category::Threes | Category::Fours | Category::Fives | Category::Sixes =>
            calculate_number_of(category as u8, &dice),
        Category::FullHouse => {
            calculate_nth_of_one(3, &dice)
        },
        Category::FourOfAKind => {
            calculate_nth_of_one(4, &dice)
        },
        Category::LittleStraight => {
            calculate_straight(1, &dice)
        },
        Category::BigStraight => {
            calculate_straight(2, &dice)
        }
        Category::Choice => {
            dice.iter().sum()
        },
        Category::Yacht => {
            if dice.iter().is_sorted() && dice[0] == dice[4] { 50 } else { 0 }
        }
    }
}
