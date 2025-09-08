use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum).flat_map(
        |a| {
            (a+1..sum-a).map(move |b| (a, b))
        }
    ).filter_map(
        |(a, b)| {
            let c = sum - a - b;
            if c > b && a + b + c == sum && a.pow(2) + b.pow(2) == c.pow(2) {
                Some([a, b, c])
            } else { None }
        }
    ).collect()
}