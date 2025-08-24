pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let mut idx = 2_u64;
    let mut nn = n;
    let sqrt = n.isqrt() as u64;

    while nn != 1 {
        if idx > sqrt {
            result.push(nn);
            break;
        }
        match nn % idx {
            0 => {
                result.push(idx);
                nn /= idx;
            }
            _ => {
                idx += 1;
            }
        }
    }

    result
}
