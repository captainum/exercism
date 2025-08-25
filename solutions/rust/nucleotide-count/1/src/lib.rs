use std::collections::HashMap;

static NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        Err(nucleotide)
    } else if let Some(ch) = dna.chars().find(|ch| !NUCLEOTIDES.contains(ch)) {
        Err(ch)
    } else {
        Ok(dna.chars().filter(|&ch| ch == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if let Some(ch) = dna.chars().find(|ch| !NUCLEOTIDES.contains(ch)) {
        return Err(ch);
    }

    let mut result = HashMap::<char, usize>::new();
    
    for ch in dna.chars() {
        *result.entry(ch).or_insert(0) += 1;
    }

    for ch in NUCLEOTIDES {
        result.entry(ch).or_insert(0);
    }

    Ok(result)
}