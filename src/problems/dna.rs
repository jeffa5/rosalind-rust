use std::convert::TryFrom;

use crate::utils::dna::{Dna, DnaBase};

pub fn solve(input: &str) -> String {
    let dna = Dna::try_from(input).unwrap();

    let (a, c, g, t) = dna
        .into_iter()
        .fold((0, 0, 0, 0), |(a, c, g, t), base| match base {
            DnaBase::A => (a + 1, c, g, t),
            DnaBase::C => (a, c + 1, g, t),
            DnaBase::G => (a, c, g + 1, t),
            DnaBase::T => (a, c, g, t + 1),
        });

    format!("{} {} {} {}", a, c, g, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC"),
            "20 12 17 21"
        )
    }
}
