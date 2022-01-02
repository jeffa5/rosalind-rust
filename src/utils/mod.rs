pub mod digraph;
pub mod dna;
pub mod fasta;
pub mod protein;
pub mod rna;
pub mod sequence;
pub mod uniprot;

pub fn read_file(filename: &str) -> String {
    std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Couldn't open file: {}", filename))
        .trim()
        .into()
}
