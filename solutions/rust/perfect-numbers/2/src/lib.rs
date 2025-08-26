#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let mut sum = 1;

    for i in 2..=num.isqrt() {
        if num % i == 0 {
            let sub = num / i;
            sum += i;
            if i != sub {
                sum += sub;
            }
        }
    }

    if sum == num && sum != 1 {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}