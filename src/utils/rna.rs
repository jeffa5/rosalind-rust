use crate::utils::dna::{Dna, DnaBase};
use crate::utils::sequence::Sequence;
use std::convert::{From, TryFrom};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RnaBase {
    A,
    C,
    G,
    U,
}

impl std::fmt::Display for RnaBase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RnaBase::A => "A",
                RnaBase::C => "C",
                RnaBase::G => "G",
                RnaBase::U => "U",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Rna {
    sequence: Vec<RnaBase>,
}

impl Rna {
    pub fn len(&self) -> usize {
        self.sequence.len()
    }

    pub fn chunks_exact(&self, chunk_size: usize) -> std::slice::ChunksExact<RnaBase> {
        self.sequence.chunks_exact(chunk_size)
    }
}

impl Sequence for Rna {}

impl std::fmt::Display for Rna {
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

impl From<Dna> for Rna {
    fn from(dna: Dna) -> Self {
        let mut rna = Vec::with_capacity(dna.len());
        for dna_base in dna {
            rna.push(match dna_base {
                DnaBase::A => RnaBase::A,
                DnaBase::C => RnaBase::C,
                DnaBase::G => RnaBase::G,
                DnaBase::T => RnaBase::U,
            })
        }
        Rna { sequence: rna }
    }
}

impl From<&Dna> for Rna {
    fn from(dna: &Dna) -> Self {
        let mut rna = Vec::with_capacity(dna.len());
        for dna_base in dna.iter() {
            rna.push(match dna_base {
                DnaBase::A => RnaBase::A,
                DnaBase::C => RnaBase::C,
                DnaBase::G => RnaBase::G,
                DnaBase::T => RnaBase::U,
            })
        }
        Rna { sequence: rna }
    }
}

impl From<&mut Dna> for Rna {
    fn from(dna: &mut Dna) -> Self {
        let mut rna = Vec::with_capacity(dna.len());
        for dna_base in dna.iter() {
            rna.push(match dna_base {
                DnaBase::A => RnaBase::A,
                DnaBase::C => RnaBase::C,
                DnaBase::G => RnaBase::G,
                DnaBase::T => RnaBase::U,
            })
        }
        Rna { sequence: rna }
    }
}

impl TryFrom<String> for Rna {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut sequence = Vec::with_capacity(value.len());
        for ch in value.chars() {
            match ch.to_ascii_lowercase() {
                'a' => sequence.push(RnaBase::A),
                'c' => sequence.push(RnaBase::C),
                'g' => sequence.push(RnaBase::G),
                'u' => sequence.push(RnaBase::U),
                c => return Err(format!("Failed to convert {} to rna base", c)),
            }
        }
        Ok(Rna { sequence })
    }
}

impl TryFrom<&str> for Rna {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut sequence = Vec::with_capacity(value.len());
        for ch in value.chars() {
            match ch.to_ascii_lowercase() {
                'a' => sequence.push(RnaBase::A),
                'c' => sequence.push(RnaBase::C),
                'g' => sequence.push(RnaBase::G),
                'u' => sequence.push(RnaBase::U),
                c => return Err(format!("Failed to convert {} to rna base", c)),
            }
        }
        Ok(Rna { sequence })
    }
}

impl IntoIterator for Rna {
    type Item = RnaBase;
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
        let string = "acgu";
        let rna = Rna::try_from(string);
        assert_eq!(
            rna,
            Ok(Rna {
                sequence: vec![RnaBase::A, RnaBase::C, RnaBase::G, RnaBase::U]
            })
        )
    }

    #[test]
    fn try_from_string_with_valid_characters_passes() {
        let string = String::from("acgu");
        let rna = Rna::try_from(string);
        assert_eq!(
            rna,
            Ok(Rna {
                sequence: vec![RnaBase::A, RnaBase::C, RnaBase::G, RnaBase::U]
            })
        )
    }

    #[test]
    fn try_from_str_with_invalid_characters_fails() {
        let string = "acdt";
        let rna = Rna::try_from(string);
        assert_eq!(rna, Err("Failed to convert d to rna base".into()))
    }

    #[test]
    fn try_from_string_with_invalid_characters_fails() {
        let string = String::from("acdt");
        let rna = Rna::try_from(string);
        assert_eq!(rna, Err("Failed to convert d to rna base".into()))
    }
}
