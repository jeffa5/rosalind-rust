use crate::utils::sequence::Sequence;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum DnaBase {
    A,
    C,
    G,
    T,
}

impl std::fmt::Display for DnaBase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DnaBase::A => "A",
                DnaBase::C => "C",
                DnaBase::G => "G",
                DnaBase::T => "T",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Dna {
    sequence: Vec<DnaBase>,
}

impl Dna {
    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn get_intron_locations(&self, introns: &[&Dna]) -> Vec<(usize, usize)> {
        let mut locations = Vec::new();
        for intron in introns {
            for i in 0..self.len() - intron.len() {
                if self.sequence[i..i + intron.len()]
                    .iter()
                    .zip(intron.iter())
                    .all(|(a, b)| a == b)
                {
                    locations.push((i, i + intron.len()))
                }
            }
        }
        locations.sort_by_key(|x| x.0);
        locations
    }

    pub fn remove_introns_join_exons(&mut self, intron_locations: &[(usize, usize)]) {
        self.sequence = self
            .sequence
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                if !intron_locations
                    .iter()
                    .any(|(start, end)| i >= *start && i < *end)
                {
                    Some(*item)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Convert this DNA sequence to it's reverse complement
    pub fn reverse_complement(&mut self) {
        let mut rev_comp = Vec::with_capacity(self.len());
        for base in self.iter().rev() {
            rev_comp.push(match base {
                DnaBase::A => DnaBase::T,
                DnaBase::C => DnaBase::G,
                DnaBase::G => DnaBase::C,
                DnaBase::T => DnaBase::A,
            })
        }
        self.sequence = rev_comp
    }

    pub fn iter(&self) -> std::slice::Iter<DnaBase> {
        self.sequence.iter()
    }

    /// Calculate the hamming distance between this DNA sequence and another
    pub fn hamming_distance(&self, other: &Dna) -> Result<u32, String> {
        if self.len() != other.len() {
            Err("Sequences not of equal length".to_string())
        } else {
            let distance: u32 = self
                .iter()
                .zip(other.iter())
                .map(|(b1, b2)| if b1 != b2 { 1 } else { 0 })
                .sum();
            Ok(distance)
        }
    }

    /// Find the locations of the substring in this DNA sequence
    /// Uses 1-based indexing as per the Rosalind problem
    pub fn substring_locations(&self, other: &Dna) -> Vec<usize> {
        let mut locations = Vec::new();
        for i in 0..(self.len() - other.len()) {
            if self.sequence[i..i + other.len()] == other.sequence[..] {
                locations.push(i + 1);
            }
        }
        locations
    }

    pub fn sequence(&self) -> &[DnaBase] {
        &self.sequence
    }

    /// Compute the gc content of the dna
    pub fn compute_gc_content(&self) -> f64 {
        let gc_count = self.iter().fold(0, |count, base| match base {
            DnaBase::C | DnaBase::G => count + 1,
            DnaBase::A | DnaBase::T => count,
        });

        f64::from(gc_count) / (self.len() as f64)
    }

    /// Get the prefix of length `k` from the dna sequence
    pub fn prefix(&self, k: usize) -> &[DnaBase] {
        &self.sequence[..k]
    }

    /// Get the suffix of length `k` from the dna sequence
    pub fn suffix(&self, k: usize) -> &[DnaBase] {
        let len = self.len();
        &self.sequence[(len - k)..]
    }

    pub fn chunks_exact(&self, chunk_size: usize) -> std::slice::ChunksExact<DnaBase> {
        self.sequence.chunks_exact(chunk_size)
    }
}

impl Sequence for Dna {}

impl std::fmt::Display for Dna {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.sequence
                .iter()
                .map(|base| format!("{}", base))
                .collect::<String>()
        )
    }
}

impl From<Vec<DnaBase>> for Dna {
    fn from(value: Vec<DnaBase>) -> Self {
        Dna { sequence: value }
    }
}

impl TryFrom<String> for Dna {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

impl TryFrom<&str> for Dna {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut sequence = Vec::with_capacity(value.len());
        for ch in value.chars() {
            match ch.to_ascii_lowercase() {
                'a' => sequence.push(DnaBase::A),
                'c' => sequence.push(DnaBase::C),
                'g' => sequence.push(DnaBase::G),
                't' => sequence.push(DnaBase::T),
                c => return Err(format!("Failed to convert {} to dna base", c)),
            }
        }
        Ok(Dna { sequence })
    }
}

impl IntoIterator for Dna {
    type Item = DnaBase;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_str_with_valid_characters_passes() {
        let string = "acgt";
        let dna = Dna::try_from(string);
        assert_eq!(
            dna,
            Ok(Dna {
                sequence: vec![DnaBase::A, DnaBase::C, DnaBase::G, DnaBase::T]
            })
        )
    }

    #[test]
    fn try_from_string_with_valid_characters_passes() {
        let string = String::from("acgt");
        let dna = Dna::try_from(string);
        assert_eq!(
            dna,
            Ok(Dna {
                sequence: vec![DnaBase::A, DnaBase::C, DnaBase::G, DnaBase::T]
            })
        )
    }

    #[test]
    fn try_from_str_with_invalid_characters_fails() {
        let string = "acdt";
        let dna = Dna::try_from(string);
        assert_eq!(dna, Err("Failed to convert d to dna base".into()))
    }

    #[test]
    fn try_from_string_with_invalid_characters_fails() {
        let string = String::from("acdt");
        let dna = Dna::try_from(string);
        assert_eq!(dna, Err("Failed to convert d to dna base".into()))
    }

    #[test]
    fn get_prefix_length_3() {
        let string = String::from("aaataaa");
        let dna = Dna::try_from(string).unwrap();
        assert_eq!(dna.prefix(3), [DnaBase::A, DnaBase::A, DnaBase::A])
    }

    #[test]
    fn get_suffix_length_3() {
        let string = String::from("aaataaa");
        let dna = Dna::try_from(string).unwrap();
        assert_eq!(dna.suffix(3), [DnaBase::A, DnaBase::A, DnaBase::A])
    }
}
