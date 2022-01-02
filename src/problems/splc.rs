use crate::utils::dna::Dna;
use crate::utils::fasta::Fasta;
use crate::utils::protein::Protein;
use crate::utils::rna::Rna;

pub fn solve(input: Fasta<Dna>) -> String {
    let mut dna = input.iter().next().unwrap().1.clone();

    let introns: Vec<_> = input.iter().skip(1).map(|p| &p.1).collect();

    let intron_locations = dna.get_intron_locations(&introns[..]);

    dna.remove_introns_join_exons(&intron_locations);

    let protein = Protein::from(Rna::from(dna));

    format!("{}", protein)
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
                    ">Rosalind_10
ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG
>Rosalind_12
ATCGGTCGAA
>Rosalind_15
ATCGGTCGAGCGTGT"
                )
                .unwrap()
            ),
            "MVYIADKQHVASREAYGHMFKVCA"
        )
    }
}
