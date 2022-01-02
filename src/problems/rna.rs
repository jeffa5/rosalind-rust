use std::convert::TryFrom;

use crate::utils::dna::Dna;
use crate::utils::rna::Rna;

pub fn solve(input: &str) -> String {
    let dna = Dna::try_from(input).unwrap();
    format!("{}", Rna::from(dna))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("GATGGAACTTGACTACGTAAATT"), "GAUGGAACUUGACUACGUAAAUU")
    }
}
