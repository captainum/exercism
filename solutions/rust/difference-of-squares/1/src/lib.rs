pub fn square_of_sum(n: u32) -> u32 {
    let mut result = 0_u32;

    for i in 1 .. n + 1 {
        result += i
    }

    result.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result = 0_u32;

    for i in 1 .. n + 1 {
        result += i.pow(2);
    }

    result
}

pub fn difference(n: u32) -> u32 {
    let mut result = 0_u32;

    for i in 1 .. n + 1 {
        for j in i + 1 .. n + 1 {
            if i != j { result += i * j * 2; }
        }
    }

    result
}
