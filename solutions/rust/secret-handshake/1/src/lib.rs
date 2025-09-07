pub fn actions(mut n: u8) -> Vec<&'static str> {
    if n > 31 {
        panic!("n is inconvenient!");
    }

    let mut result = vec![];

    let mut step = 0;

    while n != 0 {
        if n & 0x01 == 1 {
            match step {
                0 => { result.push("wink"); },
                1 => { result.push("double blink"); },
                2 => { result.push("close your eyes"); },
                3 => { result.push("jump"); },
                _ => { result.reverse(); }
            }
        }

        step += 1;
        n >>= 1;
    }

    result
}
