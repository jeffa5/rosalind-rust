use crate::utils::dna::{Dna, DnaBase};
use crate::utils::fasta::Fasta;
use itertools::Itertools;

pub fn solve(input: Fasta<Dna>) -> String {
    let len = input.iter().next().unwrap().1.len();
    let mut a = vec![0; len];
    let mut c = vec![0; len];
    let mut g = vec![0; len];
    let mut t = vec![0; len];

    for (_, sequence) in input.iter() {
        for (i, base) in sequence.iter().enumerate() {
            match base {
                DnaBase::A => a[i] += 1,
                DnaBase::C => c[i] += 1,
                DnaBase::G => g[i] += 1,
                DnaBase::T => t[i] += 1,
            }
        }
    }

    let mut consensus = Vec::new();

    for i in 0..len {
        consensus.push(
            [
                (a[i], DnaBase::A),
                (c[i], DnaBase::C),
                (g[i], DnaBase::G),
                (t[i], DnaBase::T),
            ]
            .iter()
            .max_by_key(|x| x.0)
            .unwrap()
            .1,
        )
    }

    format!(
        "{}\nA: {}\nC: {}\nG: {}\nT: {}",
        consensus.iter().join(""),
        a.iter().join(" "),
        c.iter().join(" "),
        g.iter().join(" "),
        t.iter().join(" "),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve(
                Fasta::try_from(
                    ">Rosalind_1
ATCCAGCT
>Rosalind_2
GGGCAACT
>Rosalind_3
ATGGATCT
>Rosalind_4
AAGCAACC
>Rosalind_5
TTGGAACT
>Rosalind_6
ATGCCATT
>Rosalind_7
ATGGCACT"
                )
                .unwrap()
            ),
            "ATGCAACT
A: 5 1 0 0 5 5 0 0
C: 0 0 1 4 2 0 6 1
G: 1 1 6 3 0 1 0 0
T: 1 5 0 0 0 1 1 6"
        )
    }
}
