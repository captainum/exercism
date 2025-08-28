use std::iter;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();

    if input.is_empty() {
        return result;
    }

    let mut max_row: Vec::<u64> = iter::repeat(0).take(input.len()).collect();
    let mut min_col: Vec::<u64> = iter::repeat(u64::MAX).take(input[0].len()).collect();

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] > max_row[y] {
                max_row[y] = input[y][x];
            }
            if input[y][x] < min_col[x] {
                min_col[x] = input[y][x];
            }
        }
    }
    
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == max_row[y] && input[y][x] == min_col[x] {
                result.push((y, x));
            }
        }
    }
    
    result
}