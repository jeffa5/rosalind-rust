use std::convert::TryFrom;

use crate::utils::dna::Dna;

pub fn solve(input: &str) -> String {
    let mut dna = Dna::try_from(input).unwrap();

    dna.reverse_complement();

    format!("{}", dna)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("AAAACCCGGT"), "ACCGGGTTTT")
    }
}
