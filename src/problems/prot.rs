use std::convert::TryFrom;

use crate::utils::protein::Protein;
use crate::utils::rna::Rna;

pub fn solve(input: &str) -> String {
    let rna = Rna::try_from(input).unwrap();

    let protein = Protein::from(rna);

    format!("{}", protein)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA"),
            "MAMAPRTEINSTRING"
        )
    }
}
