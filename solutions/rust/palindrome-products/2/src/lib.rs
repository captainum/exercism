use std::collections::{ HashSet, HashMap };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn is_palindrome(n: u64) -> bool {
    let str = n.to_string();

    str[.. str.len() / 2] == str.chars().rev().collect::<String>()[.. str.len() / 2]
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes = HashMap::<u64, HashSet<(u64, u64)>>::new();

    for i in min .. max + 1 {
        for j in i .. max + 1 {
            let product = i * j;
            if is_palindrome(product) {
                palindromes.entry(product).or_default().insert((i, j));
            }
        }
    }
    
    palindromes.keys().max().map(
        |&n| {
            let &m = palindromes.keys().min().unwrap();
            (
                Palindrome { value: m, factors: palindromes.get(&m).unwrap().clone() },
                Palindrome { value: n, factors: palindromes.get(&n).unwrap().clone() },
            )
        }
    )
}
