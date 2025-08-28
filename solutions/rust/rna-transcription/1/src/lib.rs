pub trait Sequence {
    const VALID_NUCLEOTIDES: [char; 4];

    fn _new(sequence: &str) -> Result<Self, usize> where Self: Sized {
        if let Some(idx) = sequence.chars().position(
            |c: char| !Self::VALID_NUCLEOTIDES.contains(&c)
        ) {
            Err(idx)
        } else {
            Ok(Self::from_sequence(sequence.to_string()))
        }
    }

    fn from_sequence(sequence: String) -> Self;
}
#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(sequence: &str) -> Result<Self, usize> {
        Self::_new(sequence)
    }

    pub fn into_rna(self) -> Rna {
        let sequence = self.sequence.chars().map(
            |ch| {
                let idx = Self::VALID_NUCLEOTIDES.iter().position(
                    |&c| c == ch
                ).unwrap();
                
                Rna::VALID_NUCLEOTIDES[idx]
            }
        ).collect::<String>();

        Rna { sequence }
    }
}

impl Rna {
    pub fn new(sequence: &str) -> Result<Self, usize> {
        Self::_new(sequence)
    }
}

impl Sequence for Dna {
    const VALID_NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];

    fn from_sequence(sequence: String) -> Self {
        Self { sequence }
    }
}

impl Sequence for Rna {
    const VALID_NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

    fn from_sequence(sequence: String) -> Self {
        Self { sequence }
    }
}