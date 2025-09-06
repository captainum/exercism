pub struct RailFence {
    rails: u32
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut tmp_res = Vec::<Vec<char>>::new();
        for _ in 0..self.rails {
            tmp_res.push(Vec::<char>::new());
        }

        let divisor = (self.rails as usize - 1) * 2;

        for chunk in text.chars().collect::<Vec<char>>().chunks(divisor) {
            let border = match divisor / 2 >= chunk.len() {
                true => chunk.len(),
                false => divisor / 2,
            };

            for (idx, ch) in chunk[..border].iter().enumerate() {
                tmp_res.get_mut(idx).unwrap().push(*ch);
            }

            if border < chunk.len() {
                for (idx, ch) in chunk[border..].iter().rev().enumerate() {
                    tmp_res.get_mut(1 + idx).unwrap().push(*ch);
                }
            }
        }

        tmp_res.iter().fold(
            String::new(),
            |res, row| {
                res + row.iter().collect::<String>().as_str()
            }
        )
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut tmp_res = Vec::<Vec<char>>::new();
        for _ in 0..self.rails {
            tmp_res.push(Vec::<char>::new());
        }

        let divisor = (self.rails as usize - 1) * 2;
        let chunks_count  = cipher.len() / divisor;
        let last_chunk_size = cipher.len() % divisor;

        let mut offset = 0;

        for idx in 0..self.rails as usize {
            let substring_size = if idx == 0 || idx == self.rails as usize - 1 {
                chunks_count + match last_chunk_size > 0 {
                    true => {
                        if idx == 0 || last_chunk_size > divisor / 2 {
                            1
                        } else {
                            0
                        }
                    },
                    false => 0,
                }
            } else {
                chunks_count * 2 + match last_chunk_size > 0 {
                    true => {
                        let mut tmp_res = 0;
                        if idx < last_chunk_size {
                            tmp_res += 1;
                        }
                        
                        if divisor - idx <= last_chunk_size {
                            tmp_res += 1;
                        }

                        tmp_res
                    },
                    false => 0,
                }
            };

            tmp_res.get_mut(idx).unwrap().append(
                &mut cipher[offset..offset + substring_size].chars().collect::<Vec<char>>()
            );

            offset += substring_size;
        }

        let mut result = String::new();
        
        for val in &mut tmp_res {
            val.reverse();
        }
        
        let mut is_done = false;
        
        loop {
            if is_done {
                break;
            }

            for item in tmp_res.iter_mut().take(self.rails as usize - 1) {
                let ch = item.pop();
                if let Some(ch) = ch {
                    result.push(ch);
                } else {
                    is_done = true;
                    break;
                }
            }
            
            for item in tmp_res.iter_mut().rev().take(self.rails as usize - 1) {
                let ch = item.pop();
                if let Some(ch) = ch {
                    result.push(ch);
                } else {
                    is_done = true;
                }
            }
        }

        result
    }
}
