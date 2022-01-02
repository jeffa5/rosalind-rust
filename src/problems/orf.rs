use itertools::Itertools;
use std::collections::BTreeSet;
use std::convert::TryFrom;

use crate::utils::dna::Dna;
use crate::utils::fasta::Fasta;
use crate::utils::protein::Protein;

fn try_from_dna(dna: &Dna) -> String {
    let protein = Protein::try_from(dna);
    if let Ok(p) = protein {
        return format!("{}", p);
    }
    "".to_string()
}

pub fn solve(mut input: Fasta<Dna>) -> String {
    let mut treeset = BTreeSet::new();

    for (_, dna) in input.iter_mut() {
        let sequence = dna.sequence();
        for i in 0..sequence.len() {
            let s = try_from_dna(&Dna::from(sequence[i..].to_vec()));
            if s != "" {
                treeset.insert(s);
            }
        }
        dna.reverse_complement();
        let sequence = dna.sequence();
        for i in 0..sequence.len() {
            let s = try_from_dna(&Dna::from(sequence[i..].to_vec()));
            if s != "" {
                treeset.insert(s);
            }
        }
    }
    for item in treeset.iter() {
        eprintln!("{:?}", item);
    }

    treeset.iter().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve(
                Fasta::try_from(
                    ">Rosalind_99
AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG"
                )
                .unwrap()
            ),
            "M
MGMTPRLGLESLLE
MLLGSFRLIPKETLIQVAGSSPCNLS
MTPRLGLESLLE"
        )
    }
}
