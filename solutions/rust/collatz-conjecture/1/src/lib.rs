pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0_u64;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            count += 1;
        }
        else {
            n = 3 * n + 1;
            count += 1;
        }
    }

    Some(count)
}
