use crate::utils::dna::Dna;
use crate::utils::fasta::Fasta;

pub fn solve(input: Fasta<Dna>) -> String {
    let (id, gc_content) = input
        .iter()
        .fold(("", 0.), |(id, gc_content), (seq_id, seq)| {
            let gc_count = seq.compute_gc_content();
            if gc_count > gc_content {
                (seq_id, gc_count)
            } else {
                (id, gc_content)
            }
        });

    format!("{}\n{:.6}", id, 100. * gc_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        use std::convert::TryFrom;

        assert_eq!(
            solve(
                Fasta::try_from(
                    ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT"
                )
                .unwrap()
            ),
            "Rosalind_0808
60.919540"
        )
    }
}
