pub fn nth(n: u32) -> u32 {
    let mut primes = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23);

    if n < primes.len() as u32 {
        return primes[n as usize];
    }

    let mut idx = 24;

    loop {
        let mut found = false;
        for i in &primes {
            if idx % i == 0 {
                found = true;
                break;
            }
        }
        if !found {
            primes.push(idx);
        }
        if n < primes.len() as u32 {
            return primes.pop().unwrap();
        }
        idx += 1;
    }
}
