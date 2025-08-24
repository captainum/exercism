use std::collections::{ HashMap, HashSet };

pub struct School {
    roster: HashMap<u32, HashSet<String>>,
}

impl School {
    pub fn new() -> School {
        Self { roster: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if !self.roster.iter().any(|(_, v)| v.contains(student)) {
            self.roster.entry(grade).or_default().insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut result = self.roster.keys().copied().collect::<Vec<u32>>(); 
        result.sort();
        
        result
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        if !self.roster.contains_key(&grade) {
            Vec::new()
        }
        else {
            let mut result = self.roster.get(&grade).unwrap().iter().map(
                |s| s.to_string()
            ).collect::<Vec<String>>();
            result.sort();

            result
        }
    }
}
