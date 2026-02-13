pub struct Matrix {
    values: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        let values = input.split('\n').flat_map(
            |line| {
                line.split_whitespace().map(
                    |value| {
                        match value.parse::<u32>() {
                            Ok(value) => value,
                            Err(e) => panic!("parse error: {}", e),
                        }
                    }
                )
            }
        ).collect::<Vec<u32>>();

        let rows = input.lines().count();
        let cols = values.len() / rows;

        Self { values, rows, cols }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.rows {
            None
        } else {
            Some(
                (0..self.cols).map(
                    |col_no| {
                        self.values[(row_no - 1) * self.cols + col_no]
                    }
                ).collect()
            )
        }
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > self.cols {
            None
        } else {
            Some(
                (0..self.rows).map(
                    |row_no| {
                        self.values[row_no * self.cols + col_no - 1]
                    }
                ).collect()
            )
        }
    }
}
