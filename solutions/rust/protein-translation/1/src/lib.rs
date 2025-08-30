use std::collections::HashMap;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let dict: HashMap::<&str, &str> = HashMap::from(
    [
            ("AUG", "Methionine"),
            ("UUU", "Phenylalanine"),
            ("UUC", "Phenylalanine"),
            ("UUA", "Leucine"),
            ("UUG", "Leucine"),
            ("UCU", "Serine"),
            ("UCC", "Serine"),
            ("UCA", "Serine"),
            ("UCG", "Serine"),
            ("UAU", "Tyrosine"),
            ("UAC", "Tyrosine"),
            ("UGU", "Cysteine"),
            ("UGC", "Cysteine"),
            ("UGG", "Tryptophan"),
            ("UAA", "STOP"),
            ("UAG", "STOP"),
            ("UGA", "STOP"),
        ]
    );

    let mut result = Vec::new();
    
    let mut has_stop = false;
    for chars in rna.chars().collect::<Vec<char>>().chunks_exact(3) {
        let key = chars.iter().collect::<String>();
        if let Some(&val) = dict.get(key.as_str()) {
            if val == "STOP" {
                has_stop = true;
                break;
            } else {
                result.push(val);
            }
        } else {
            return None;
        }
    }
    
    if !has_stop && rna.len() % 3 != 0 {
        None
    } else {
        Some(result)
    }
}
