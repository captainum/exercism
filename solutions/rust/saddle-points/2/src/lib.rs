use std::iter;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();

    if input.is_empty() {
        return result;
    }

    let mut max_row: Vec::<u64> = iter::repeat_n(0, input.len()).collect();
    let mut min_col: Vec::<u64> = iter::repeat_n(u64::MAX, input[0].len()).collect();

    for (y, row) in input.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val > max_row[y] {
                max_row[y] = val;
            }
            if val < min_col[x] {
                min_col[x] = val;
            }
        }
    }

    for (y, row) in input.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            if val == max_row[y] && val == min_col[x] {
                result.push((y, x));
            }
        }
    }

    result
}