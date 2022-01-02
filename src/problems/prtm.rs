use crate::utils::protein::Protein;
use std::convert::TryFrom;

pub fn solve(input: &str) -> String {
    let protein = Protein::try_from(input).unwrap();

    let mass = protein.calculate_mass();

    format!("{:.3}", mass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve("SKADYEK"), "821.392")
    }
}
