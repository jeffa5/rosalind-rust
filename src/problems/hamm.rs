use std::convert::TryFrom;

use crate::utils::dna::Dna;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();

    let dna1 = Dna::try_from(lines.next().unwrap()).unwrap();
    let dna2 = Dna::try_from(lines.next().unwrap()).unwrap();

    let distance = dna1.hamming_distance(&dna2).unwrap();

    format!("{}", distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve(
                "GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT"
            ),
            "7"
        )
    }
}
