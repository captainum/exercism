pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(mut row_count: u32) -> Self {
        let mut rows = Vec::<Vec<u32>>::new();

        while row_count > 0 {
            if rows.is_empty() {
                rows.push(vec![1]);
            } else {
                let last_row = rows.last().unwrap();
                let mut new_row = Vec::<u32>::new();

                for (idx, &val) in last_row.iter().enumerate() {
                    if idx == 0 {
                        new_row.push(val);
                    } else {
                        new_row.push(val + last_row[idx - 1]);
                    }
                }

                new_row.push(1);

                rows.push(new_row);
            }
            
            row_count -= 1;
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}