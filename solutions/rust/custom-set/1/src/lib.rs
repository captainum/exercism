use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    buckets: Vec<Vec<T>>,
}

fn calculate_hash<T>(value: &T) -> usize
where T: Hash {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);

    hasher.finish() as usize
}

impl<T> CustomSet<T> where T: Hash + Clone + Copy + Eq {
    pub fn new(_input: &[T]) -> Self {
        let buckets = (0..1024).map(|_| Vec::<T>::new()).collect::<Vec<_>>();

        let mut result = Self { buckets };

        for &elem in _input {
            result.add(elem);
        }

        result
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.buckets.get(calculate_hash(_element) % 1024).unwrap().contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        let hash = calculate_hash(&_element);

        let bucket = self.buckets.get_mut(hash % 1024).unwrap();

        if !bucket.contains(&_element) {
            bucket.push(_element);
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.buckets.iter().all(
            |bucket| {
                bucket.iter().all(|elem| _other.contains(elem))
            }
        )
    }

    pub fn is_empty(&self) -> bool {
        self.buckets.iter().all(|bucket| bucket.is_empty())
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !self.buckets.iter().any(
            |bucket| {
                bucket.iter().any(|elem| _other.contains(elem))
            }
        )
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let mut result = Self::new(&[]);

        for bucket in self.buckets.iter() {
            for elem in bucket {
                if _other.contains(elem) {
                    result.add(elem.clone());
                }
            }
        }
        
        result
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let mut result = Self::new(&[]);

        for bucket in self.buckets.iter() {
            for elem in bucket {
                if !_other.contains(elem) {
                    result.add(elem.clone());
                }
            }
        }

        result
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut result = Self::new(&[]);

        for bucket in self.buckets.iter() {
            for elem in bucket {
                result.add(elem.clone());
            }
        }

        for bucket in _other.buckets.iter() {
            for elem in bucket {
                result.add(elem.clone());
            }
        }

        result
    }
}