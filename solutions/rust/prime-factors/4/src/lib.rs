pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    let mut idx = 2_u64;

    while n != 1 {
        if idx > n.isqrt() {
            result.push(n);
            break;
        }

        while n % idx == 0 {
            result.push(idx);
            n /= idx;
        }
        idx += 1;
    }

    result
}
