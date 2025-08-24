#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    if first_list_len == 0 && second_list_len == 0 {
        return Comparison::Equal;
    }
    else if first_list_len == 0 {
        return Comparison::Sublist;
    }
    else if second_list_len == 0 {
        return Comparison::Superlist;
    }
    
    let func = |fl: &[i32], sl: &[i32]| {
        let fl_len = fl.len();
        let sl_len = sl.len();
        
        for sl_idx in 0 .. sl_len - fl_len + 1 {
            if fl == &sl[sl_idx .. sl_idx + fl_len] {
                return Comparison::Sublist;
            }
        }
        
        Comparison::Unequal
    };
    
    if first_list_len < second_list_len {
        func(first_list, second_list)
    }
    else {
        match func(second_list, first_list) {
            Comparison::Sublist => {
                if first_list_len == second_list_len {
                    Comparison::Equal
                }
                else { Comparison::Superlist }
            }
            _ => Comparison::Unequal
        }
    }
}
