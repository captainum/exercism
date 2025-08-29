pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    let mut result = vec![vec![0; size as usize]; size as usize];

    for idx in 0..size * size {
        println!("x={x}, y={y}, idx={idx}");
        result[y as usize][x as usize] = idx + 1;
        
        let right_is_done = x == size - 1 || result[y as usize][x as usize + 1] != 0;
        let down_is_done = y == size - 1 || result[y as usize + 1][x as usize] != 0;
        let left_is_done = x == 0 || result[y as usize][x as usize - 1] != 0;
        let up_is_done = y == 0 || result[y as usize - 1][x as usize] != 0;

        println!("right_is_done={right_is_done}, down_is_done={down_is_done}, left_is_done={left_is_done}, up_is_done={up_is_done}");
        
        if right_is_done && up_is_done && y != size - 1 && result[y as usize + 1][x as usize] == 0 {
            x = x;
            y = y + 1;
        } else if right_is_done && down_is_done && x != 0 && result[y as usize][x as usize - 1] == 0  {
            x = x - 1;
            y = y;
        } else if left_is_done && down_is_done && y != 0 && result[y as usize - 1][x as usize] == 0  {
            x = x;
            y = y - 1;
        } else if left_is_done && up_is_done && x != size - 1 && result[y as usize][x as usize + 1] == 0  {
            x = x + 1;
            y = y;
        } else if right_is_done && y != size - 1 && result[y as usize + 1][x as usize] == 0  {
            x = x;
            y = y + 1;
        } else if down_is_done && x != 0 {
            x = x - 1;
            y = y;
        } else if left_is_done && y != 0 {
            x = x;
            y = y - 1;
        } else {
            x = x + 1;
            y = y;
        }
    }

    result
}