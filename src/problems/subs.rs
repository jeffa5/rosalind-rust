use std::convert::TryFrom;

use crate::utils::dna::Dna;

pub fn solve(input: &str) -> String {
    let mut lines = input.lines();
    let dna1 = Dna::try_from(lines.next().unwrap()).unwrap();
    let dna2 = Dna::try_from(lines.next().unwrap()).unwrap();

    let locations = dna1.substring_locations(&dna2);

    let mut output_str = String::new();
    for location in locations {
        output_str.push_str(&format!("{} ", location))
    }
    output_str.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(
            solve(
                "GATATATGCATATACTT
ATAT"
            ),
            "2 4 10"
        )
    }
}
