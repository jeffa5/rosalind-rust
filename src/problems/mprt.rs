use crate::utils::protein::ProteinMotif;
use crate::utils::uniprot;
use itertools::Itertools;

pub fn solve(input: &str) -> String {
    let n_glycosylation_motif = ProteinMotif::new("N{P}[ST]{P}");
    let mut output = String::new();
    for id in input.lines() {
        let fasta = uniprot::get_fasta(id);
        for (_, sequence) in fasta.iter() {
            let locations = sequence.find_motif_locations(&n_glycosylation_motif);
            if !locations.is_empty() {
                output.push_str(&format!("{}\n{}\n", id, locations.iter().join(" ")))
            }
        }
    }
    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve(
                "A2Z669
B5ZC00
P07204_TRBM_HUMAN
P20840_SAG1_YEAST"
            ),
            "B5ZC00
85 118 142 306 395
P07204_TRBM_HUMAN
47 115 116 382 409
P20840_SAG1_YEAST
79 109 135 248 306 348 364 402 485 501 614"
        )
    }
}
