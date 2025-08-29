pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marks = Vec::<bool>::with_capacity(upper_bound as usize);

    if upper_bound < 2 {
        return Vec::new();
    }
    
    for _ in 0..2 {
        marks.push(true);
    }

    for _ in 2..=upper_bound {
        marks.push(false);
    }

    for i in 2..=upper_bound {
        if marks[i as usize] {
            continue;
        }

        let mut tmp = i * 2;
        while tmp <= upper_bound {
            marks[tmp as usize] = true;
            tmp += i;
        }
    }

    marks.iter().enumerate().filter_map(
        |(idx, &val)| {
            match val {
                true => None,
                false => Some(idx as u64),
            }
        }
    ).collect::<Vec<u64>>()
}