use crate::utils::protein::Protein;
use std::convert::TryFrom;

pub fn solve(input: &str) -> String {
    let protein = Protein::try_from(input).unwrap();
    let count = protein.calculate_potential_mrna_count(1_000_000);

    format!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("MA"), "12")
    }
}
