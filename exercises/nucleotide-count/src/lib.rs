use std::collections::HashMap;
use std::str::FromStr;

const BASES: [char; 4] = ['A','C','T','G'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // next iteration use fold!
    let dna_strand = String::from_str(dna).unwrap();
    if BASES.contains(&nucleotide) {

        let error_base = dna_strand.chars().filter(|c: &char| !BASES.contains(&c)).next();
        
        if let Some(error_base) = error_base {
            return Err(error_base);
        }
        let matches = dna_strand.chars().filter(|c: &char| *c == nucleotide);
        return Ok(matches.count()); 
    }
    Err(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    );
}
