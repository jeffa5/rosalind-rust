use crate::utils::digraph::Digraph;
use crate::utils::dna::Dna;
use crate::utils::fasta::Fasta;

pub fn solve(input: Fasta<Dna>) -> String {
    let k = 3;
    let graph: Digraph<&Dna> = Digraph::overlap_graph_from_fasta_dna(&input, k);
    let list = graph.adjacency_list();

    let mut s = String::new();

    for (node1, node2) in list {
        s.push_str(&format!("{} {}\n", node1.name(), node2.name()))
    }
    s.trim().to_string()
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
                    ">Rosalind_0498
AAATAAA
>Rosalind_2391
AAATTTT
>Rosalind_2323
TTTTCCC
>Rosalind_0442
AAATCCC
>Rosalind_5013
GGGTGGG"
                )
                .unwrap()
            ),
            "Rosalind_0498 Rosalind_0442
Rosalind_0498 Rosalind_2391
Rosalind_2391 Rosalind_2323"
        )
    }
}
