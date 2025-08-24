#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let func = |fl: &[i32], sl: &[i32]| {
        for sl_idx in 0 .. sl.len() - fl.len() + 1 {
            if fl == &sl[sl_idx .. sl_idx + fl.len()] {
                return Comparison::Sublist;
            }
        }

        Comparison::Unequal
    };
    
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (first_list_len, second_list_len) if first_list_len < second_list_len => {
            func(first_list, second_list)
        },
        (first_list_len, second_list_len) if first_list_len > second_list_len => {
            match func(second_list, first_list) {
                Comparison::Sublist => Comparison::Superlist,
                other => other,
            }
        },
        (_, _) => if first_list == second_list { Comparison::Equal } else { Comparison::Unequal },
    }
}
