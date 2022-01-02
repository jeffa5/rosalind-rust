use crate::utils::fasta::Fasta;
use crate::utils::protein::Protein;
use std::convert::TryFrom;

static UNIPROT_BASE_URL: &str = "https://www.uniprot.org/uniprot/";

pub fn get_fasta(id: &str) -> Fasta<Protein> {
    let url = &format!("{}{}.fasta", UNIPROT_BASE_URL, id);
    let fasta_data = reqwest::get(url)
        .expect("Failed to reach uniprot url")
        .text()
        .expect("Failed to unwrap text");

    Fasta::try_from(fasta_data).unwrap()
}
