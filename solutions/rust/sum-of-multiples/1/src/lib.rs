use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    
    for factor in factors {
        if *factor == 0 { continue; }
        let mut x = factor.clone();
        while x < limit {
            set.insert(x);
            x += factor;
        }
    }
    
    set.iter().sum()
}
