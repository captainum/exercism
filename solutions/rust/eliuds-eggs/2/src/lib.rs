pub fn egg_count(mut display_value: u32) -> usize {
    let mut result: usize = 0;

    while display_value != 0 {
        result += (display_value & 0b1) as usize;
        display_value >>= 1;
    }

    result
}
