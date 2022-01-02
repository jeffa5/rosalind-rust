use crate::utils::dna::{Dna, DnaBase};
use itertools::Itertools;
use std::convert::TryFrom;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let dna = Dna::try_from(lines.next().unwrap()).unwrap();
    let array = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(str::parse::<f64>)
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let mut output: Vec<f64> = Vec::new();
    for gc_content in array {
        output.push(
            dna.iter()
                .map(|base| {
                    (match base {
                        DnaBase::A => (1. - gc_content) / 2.,
                        DnaBase::C => gc_content / 2.,
                        DnaBase::G => gc_content / 2.,
                        DnaBase::T => (1. - gc_content) / 2.,
                    })
                    .log10()
                })
                .sum(),
        )
    }

    output
        .iter()
        .map(|x| format!("{:.3}", x))
        .join(" ")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve(
                "ACGATACAA
0.129 0.287 0.423 0.476 0.641 0.742 0.783
"
            ),
            "-5.737 -5.217 -5.263 -5.360 -5.958 -6.628 -7.009"
        )
    }
}
