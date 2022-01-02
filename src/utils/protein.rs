use crate::utils::dna::{Dna, DnaBase};
use crate::utils::rna::{Rna, RnaBase};
use crate::utils::sequence::Sequence;
use std::convert::TryFrom;

enum Match {
    Exact(AminoAcid),
    AnyExcept(AminoAcid),
    Union(Vec<AminoAcid>),
}

pub struct ProteinMotif {
    pattern: Vec<Match>,
}

impl ProteinMotif {
    pub fn new(s: &str) -> Self {
        let mut in_any_except = false;
        let mut in_union = false;
        let mut union = Vec::new();

        let mut pattern = Vec::new();

        for c in s.chars() {
            match c {
                '{' => in_any_except = true,
                '}' => in_any_except = false,
                '[' => in_union = true,
                ']' => {
                    in_union = false;
                    pattern.push(Match::Union(union));
                    union = Vec::new()
                }
                c => {
                    if let Ok(aa) = AminoAcid::try_from(c) {
                        if in_any_except {
                            pattern.push(Match::AnyExcept(aa))
                        } else if in_union {
                            union.push(aa)
                        } else {
                            pattern.push(Match::Exact(aa))
                        }
                    }
                }
            }
        }

        ProteinMotif { pattern }
    }

    pub fn len(&self) -> usize {
        self.pattern.len()
    }

    /// Test whether this pattern matches the given slice
    fn matches(&self, sequence: &[AminoAcid]) -> bool {
        for (match_, aa) in self.pattern.iter().zip(sequence.iter()) {
            match match_ {
                Match::Exact(a) => {
                    if a != aa {
                        return false;
                    }
                }
                Match::AnyExcept(a) => {
                    if a == aa {
                        return false;
                    }
                }
                Match::Union(aas) => {
                    if !aas.contains(aa) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum AminoAcid {
    A,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    S,
    T,
    V,
    W,
    Y,
    Stop,
}

impl TryFrom<char> for AminoAcid {
    type Error = String;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'A' => Ok(AminoAcid::A),
            'C' => Ok(AminoAcid::C),
            'D' => Ok(AminoAcid::D),
            'E' => Ok(AminoAcid::E),
            'F' => Ok(AminoAcid::F),
            'G' => Ok(AminoAcid::G),
            'H' => Ok(AminoAcid::H),
            'I' => Ok(AminoAcid::I),
            'K' => Ok(AminoAcid::K),
            'L' => Ok(AminoAcid::L),
            'M' => Ok(AminoAcid::M),
            'N' => Ok(AminoAcid::N),
            'P' => Ok(AminoAcid::P),
            'Q' => Ok(AminoAcid::Q),
            'R' => Ok(AminoAcid::R),
            'S' => Ok(AminoAcid::S),
            'T' => Ok(AminoAcid::T),
            'V' => Ok(AminoAcid::V),
            'W' => Ok(AminoAcid::W),
            'Y' => Ok(AminoAcid::Y),
            _ => Err("Invalid amino acid".to_string()),
        }
    }
}

impl std::fmt::Display for AminoAcid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                AminoAcid::A => "A",
                AminoAcid::C => "C",
                AminoAcid::D => "D",
                AminoAcid::E => "E",
                AminoAcid::F => "F",
                AminoAcid::G => "G",
                AminoAcid::H => "H",
                AminoAcid::I => "I",
                AminoAcid::K => "K",
                AminoAcid::L => "L",
                AminoAcid::M => "M",
                AminoAcid::N => "N",
                AminoAcid::P => "P",
                AminoAcid::Q => "Q",
                AminoAcid::R => "R",
                AminoAcid::S => "S",
                AminoAcid::T => "T",
                AminoAcid::V => "V",
                AminoAcid::W => "W",
                AminoAcid::Y => "Y",
                AminoAcid::Stop => "",
            }
        )
    }
}

#[derive(Debug)]
pub struct Protein {
    sequence: Vec<AminoAcid>,
}

impl Protein {
    pub fn calculate_mass(&self) -> f64 {
        let mut mass = 0.;
        for aa in &self.sequence {
            mass += match aa {
                AminoAcid::A => 71.03711,
                AminoAcid::C => 103.00919,
                AminoAcid::D => 115.02694,
                AminoAcid::E => 129.04259,
                AminoAcid::F => 147.06841,
                AminoAcid::G => 57.02146,
                AminoAcid::H => 137.05891,
                AminoAcid::I => 113.08406,
                AminoAcid::K => 128.09496,
                AminoAcid::L => 113.08406,
                AminoAcid::M => 131.04049,
                AminoAcid::N => 114.04293,
                AminoAcid::P => 97.05276,
                AminoAcid::Q => 128.05858,
                AminoAcid::R => 156.10111,
                AminoAcid::S => 87.03203,
                AminoAcid::T => 101.04768,
                AminoAcid::V => 99.06841,
                AminoAcid::W => 186.07931,
                AminoAcid::Y => 163.06333,
                AminoAcid::Stop => 0.,
            }
        }
        mass
    }

    pub fn calculate_potential_mrna_count(&self, modulo: u32) -> u32 {
        self.sequence
            .iter()
            .map(|aa| match aa {
                AminoAcid::A => 4,
                AminoAcid::C => 2,
                AminoAcid::D => 2,
                AminoAcid::E => 2,
                AminoAcid::F => 2,
                AminoAcid::G => 4,
                AminoAcid::H => 2,
                AminoAcid::I => 3,
                AminoAcid::K => 2,
                AminoAcid::L => 6,
                AminoAcid::M => 1,
                AminoAcid::N => 2,
                AminoAcid::P => 4,
                AminoAcid::Q => 2,
                AminoAcid::R => 6,
                AminoAcid::S => 6,
                AminoAcid::T => 4,
                AminoAcid::V => 4,
                AminoAcid::W => 1,
                AminoAcid::Y => 2,
                AminoAcid::Stop => 3,
            })
            .fold(1, |acc, n| acc * n % modulo)
    }

    pub fn find_motif_locations(&self, motif: &ProteinMotif) -> Vec<usize> {
        let mut locations = Vec::new();

        for (i, candidate) in (&self.sequence).windows(motif.len()).enumerate() {
            if motif.matches(candidate) {
                locations.push(i + 1)
            }
        }

        locations
    }
}

impl Sequence for Protein {}

impl std::fmt::Display for Protein {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
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

impl From<Rna> for Protein {
    fn from(rna: Rna) -> Self {
        let mut protein = Vec::with_capacity(rna.len());
        for base_triple in rna.chunks_exact(3) {
            let base_triple = (base_triple[0], base_triple[1], base_triple[2]);
            protein.push(match base_triple {
                (RnaBase::U, RnaBase::U, RnaBase::U) => AminoAcid::F,
                (RnaBase::C, RnaBase::U, RnaBase::U) => AminoAcid::L,
                (RnaBase::A, RnaBase::U, RnaBase::U) => AminoAcid::I,
                (RnaBase::G, RnaBase::U, RnaBase::U) => AminoAcid::V,
                (RnaBase::U, RnaBase::U, RnaBase::C) => AminoAcid::F,
                (RnaBase::C, RnaBase::U, RnaBase::C) => AminoAcid::L,
                (RnaBase::A, RnaBase::U, RnaBase::C) => AminoAcid::I,
                (RnaBase::G, RnaBase::U, RnaBase::C) => AminoAcid::V,
                (RnaBase::U, RnaBase::U, RnaBase::A) => AminoAcid::L,
                (RnaBase::C, RnaBase::U, RnaBase::A) => AminoAcid::L,
                (RnaBase::A, RnaBase::U, RnaBase::A) => AminoAcid::I,
                (RnaBase::G, RnaBase::U, RnaBase::A) => AminoAcid::V,
                (RnaBase::U, RnaBase::U, RnaBase::G) => AminoAcid::L,
                (RnaBase::C, RnaBase::U, RnaBase::G) => AminoAcid::L,
                (RnaBase::A, RnaBase::U, RnaBase::G) => AminoAcid::M,
                (RnaBase::G, RnaBase::U, RnaBase::G) => AminoAcid::V,
                (RnaBase::U, RnaBase::C, RnaBase::U) => AminoAcid::S,
                (RnaBase::C, RnaBase::C, RnaBase::U) => AminoAcid::P,
                (RnaBase::A, RnaBase::C, RnaBase::U) => AminoAcid::T,
                (RnaBase::G, RnaBase::C, RnaBase::U) => AminoAcid::A,
                (RnaBase::U, RnaBase::C, RnaBase::C) => AminoAcid::S,
                (RnaBase::C, RnaBase::C, RnaBase::C) => AminoAcid::P,
                (RnaBase::A, RnaBase::C, RnaBase::C) => AminoAcid::T,
                (RnaBase::G, RnaBase::C, RnaBase::C) => AminoAcid::A,
                (RnaBase::U, RnaBase::C, RnaBase::A) => AminoAcid::S,
                (RnaBase::C, RnaBase::C, RnaBase::A) => AminoAcid::P,
                (RnaBase::A, RnaBase::C, RnaBase::A) => AminoAcid::T,
                (RnaBase::G, RnaBase::C, RnaBase::A) => AminoAcid::A,
                (RnaBase::U, RnaBase::C, RnaBase::G) => AminoAcid::S,
                (RnaBase::C, RnaBase::C, RnaBase::G) => AminoAcid::P,
                (RnaBase::A, RnaBase::C, RnaBase::G) => AminoAcid::T,
                (RnaBase::G, RnaBase::C, RnaBase::G) => AminoAcid::A,
                (RnaBase::U, RnaBase::A, RnaBase::U) => AminoAcid::Y,
                (RnaBase::C, RnaBase::A, RnaBase::U) => AminoAcid::H,
                (RnaBase::A, RnaBase::A, RnaBase::U) => AminoAcid::N,
                (RnaBase::G, RnaBase::A, RnaBase::U) => AminoAcid::D,
                (RnaBase::U, RnaBase::A, RnaBase::C) => AminoAcid::Y,
                (RnaBase::C, RnaBase::A, RnaBase::C) => AminoAcid::H,
                (RnaBase::A, RnaBase::A, RnaBase::C) => AminoAcid::N,
                (RnaBase::G, RnaBase::A, RnaBase::C) => AminoAcid::D,
                (RnaBase::U, RnaBase::A, RnaBase::A) => AminoAcid::Stop,
                (RnaBase::C, RnaBase::A, RnaBase::A) => AminoAcid::Q,
                (RnaBase::A, RnaBase::A, RnaBase::A) => AminoAcid::K,
                (RnaBase::G, RnaBase::A, RnaBase::A) => AminoAcid::E,
                (RnaBase::U, RnaBase::A, RnaBase::G) => AminoAcid::Stop,
                (RnaBase::C, RnaBase::A, RnaBase::G) => AminoAcid::Q,
                (RnaBase::A, RnaBase::A, RnaBase::G) => AminoAcid::K,
                (RnaBase::G, RnaBase::A, RnaBase::G) => AminoAcid::E,
                (RnaBase::U, RnaBase::G, RnaBase::U) => AminoAcid::C,
                (RnaBase::C, RnaBase::G, RnaBase::U) => AminoAcid::R,
                (RnaBase::A, RnaBase::G, RnaBase::U) => AminoAcid::S,
                (RnaBase::G, RnaBase::G, RnaBase::U) => AminoAcid::G,
                (RnaBase::U, RnaBase::G, RnaBase::C) => AminoAcid::C,
                (RnaBase::C, RnaBase::G, RnaBase::C) => AminoAcid::R,
                (RnaBase::A, RnaBase::G, RnaBase::C) => AminoAcid::S,
                (RnaBase::G, RnaBase::G, RnaBase::C) => AminoAcid::G,
                (RnaBase::U, RnaBase::G, RnaBase::A) => AminoAcid::Stop,
                (RnaBase::C, RnaBase::G, RnaBase::A) => AminoAcid::R,
                (RnaBase::A, RnaBase::G, RnaBase::A) => AminoAcid::R,
                (RnaBase::G, RnaBase::G, RnaBase::A) => AminoAcid::G,
                (RnaBase::U, RnaBase::G, RnaBase::G) => AminoAcid::W,
                (RnaBase::C, RnaBase::G, RnaBase::G) => AminoAcid::R,
                (RnaBase::A, RnaBase::G, RnaBase::G) => AminoAcid::R,
                (RnaBase::G, RnaBase::G, RnaBase::G) => AminoAcid::G,
            })
        }
        if *protein.last().unwrap() != AminoAcid::Stop {
            protein.push(AminoAcid::Stop)
        }
        Protein { sequence: protein }
    }
}

impl TryFrom<&Dna> for Protein {
    type Error = String;

    fn try_from(dna: &Dna) -> Result<Self, Self::Error> {
        let mut protein = Vec::with_capacity(dna.len());
        let mut active = false;

        for base_triple in dna.chunks_exact(3) {
            let base_triple = (base_triple[0], base_triple[1], base_triple[2]);
            if !active {
                if let (DnaBase::A, DnaBase::T, DnaBase::G) = base_triple {
                    active = true;
                    protein.push(AminoAcid::M);
                }
            } else {
                let aa = match base_triple {
                    (DnaBase::T, DnaBase::T, DnaBase::T) => AminoAcid::F,
                    (DnaBase::C, DnaBase::T, DnaBase::T) => AminoAcid::L,
                    (DnaBase::A, DnaBase::T, DnaBase::T) => AminoAcid::I,
                    (DnaBase::G, DnaBase::T, DnaBase::T) => AminoAcid::V,
                    (DnaBase::T, DnaBase::T, DnaBase::C) => AminoAcid::F,
                    (DnaBase::C, DnaBase::T, DnaBase::C) => AminoAcid::L,
                    (DnaBase::A, DnaBase::T, DnaBase::C) => AminoAcid::I,
                    (DnaBase::G, DnaBase::T, DnaBase::C) => AminoAcid::V,
                    (DnaBase::T, DnaBase::T, DnaBase::A) => AminoAcid::L,
                    (DnaBase::C, DnaBase::T, DnaBase::A) => AminoAcid::L,
                    (DnaBase::A, DnaBase::T, DnaBase::A) => AminoAcid::I,
                    (DnaBase::G, DnaBase::T, DnaBase::A) => AminoAcid::V,
                    (DnaBase::T, DnaBase::T, DnaBase::G) => AminoAcid::L,
                    (DnaBase::C, DnaBase::T, DnaBase::G) => AminoAcid::L,
                    (DnaBase::A, DnaBase::T, DnaBase::G) => AminoAcid::M,
                    (DnaBase::G, DnaBase::T, DnaBase::G) => AminoAcid::V,
                    (DnaBase::T, DnaBase::C, DnaBase::T) => AminoAcid::S,
                    (DnaBase::C, DnaBase::C, DnaBase::T) => AminoAcid::P,
                    (DnaBase::A, DnaBase::C, DnaBase::T) => AminoAcid::T,
                    (DnaBase::G, DnaBase::C, DnaBase::T) => AminoAcid::A,
                    (DnaBase::T, DnaBase::C, DnaBase::C) => AminoAcid::S,
                    (DnaBase::C, DnaBase::C, DnaBase::C) => AminoAcid::P,
                    (DnaBase::A, DnaBase::C, DnaBase::C) => AminoAcid::T,
                    (DnaBase::G, DnaBase::C, DnaBase::C) => AminoAcid::A,
                    (DnaBase::T, DnaBase::C, DnaBase::A) => AminoAcid::S,
                    (DnaBase::C, DnaBase::C, DnaBase::A) => AminoAcid::P,
                    (DnaBase::A, DnaBase::C, DnaBase::A) => AminoAcid::T,
                    (DnaBase::G, DnaBase::C, DnaBase::A) => AminoAcid::A,
                    (DnaBase::T, DnaBase::C, DnaBase::G) => AminoAcid::S,
                    (DnaBase::C, DnaBase::C, DnaBase::G) => AminoAcid::P,
                    (DnaBase::A, DnaBase::C, DnaBase::G) => AminoAcid::T,
                    (DnaBase::G, DnaBase::C, DnaBase::G) => AminoAcid::A,
                    (DnaBase::T, DnaBase::A, DnaBase::T) => AminoAcid::Y,
                    (DnaBase::C, DnaBase::A, DnaBase::T) => AminoAcid::H,
                    (DnaBase::A, DnaBase::A, DnaBase::T) => AminoAcid::N,
                    (DnaBase::G, DnaBase::A, DnaBase::T) => AminoAcid::D,
                    (DnaBase::T, DnaBase::A, DnaBase::C) => AminoAcid::Y,
                    (DnaBase::C, DnaBase::A, DnaBase::C) => AminoAcid::H,
                    (DnaBase::A, DnaBase::A, DnaBase::C) => AminoAcid::N,
                    (DnaBase::G, DnaBase::A, DnaBase::C) => AminoAcid::D,
                    (DnaBase::T, DnaBase::A, DnaBase::A) => AminoAcid::Stop,
                    (DnaBase::C, DnaBase::A, DnaBase::A) => AminoAcid::Q,
                    (DnaBase::A, DnaBase::A, DnaBase::A) => AminoAcid::K,
                    (DnaBase::G, DnaBase::A, DnaBase::A) => AminoAcid::E,
                    (DnaBase::T, DnaBase::A, DnaBase::G) => AminoAcid::Stop,
                    (DnaBase::C, DnaBase::A, DnaBase::G) => AminoAcid::Q,
                    (DnaBase::A, DnaBase::A, DnaBase::G) => AminoAcid::K,
                    (DnaBase::G, DnaBase::A, DnaBase::G) => AminoAcid::E,
                    (DnaBase::T, DnaBase::G, DnaBase::T) => AminoAcid::C,
                    (DnaBase::C, DnaBase::G, DnaBase::T) => AminoAcid::R,
                    (DnaBase::A, DnaBase::G, DnaBase::T) => AminoAcid::S,
                    (DnaBase::G, DnaBase::G, DnaBase::T) => AminoAcid::G,
                    (DnaBase::T, DnaBase::G, DnaBase::C) => AminoAcid::C,
                    (DnaBase::C, DnaBase::G, DnaBase::C) => AminoAcid::R,
                    (DnaBase::A, DnaBase::G, DnaBase::C) => AminoAcid::S,
                    (DnaBase::G, DnaBase::G, DnaBase::C) => AminoAcid::G,
                    (DnaBase::T, DnaBase::G, DnaBase::A) => AminoAcid::Stop,
                    (DnaBase::C, DnaBase::G, DnaBase::A) => AminoAcid::R,
                    (DnaBase::A, DnaBase::G, DnaBase::A) => AminoAcid::R,
                    (DnaBase::G, DnaBase::G, DnaBase::A) => AminoAcid::G,
                    (DnaBase::T, DnaBase::G, DnaBase::G) => AminoAcid::W,
                    (DnaBase::C, DnaBase::G, DnaBase::G) => AminoAcid::R,
                    (DnaBase::A, DnaBase::G, DnaBase::G) => AminoAcid::R,
                    (DnaBase::G, DnaBase::G, DnaBase::G) => AminoAcid::G,
                };

                protein.push(aa);
                if aa == AminoAcid::Stop {
                    break;
                }
            }
        }
        if !active
            || *protein.last().unwrap() != AminoAcid::Stop
            || protein == vec![AminoAcid::Stop]
        {
            return Err("No start or stop codon".to_string());
        }
        Ok(Protein { sequence: protein })
    }
}

impl TryFrom<String> for Protein {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut sequence = Vec::with_capacity(s.len());

        for c in s.chars() {
            sequence.push(match c.to_ascii_lowercase() {
                'a' => AminoAcid::A,
                'c' => AminoAcid::C,
                'd' => AminoAcid::D,
                'e' => AminoAcid::E,
                'f' => AminoAcid::F,
                'g' => AminoAcid::G,
                'h' => AminoAcid::H,
                'i' => AminoAcid::I,
                'k' => AminoAcid::K,
                'l' => AminoAcid::L,
                'm' => AminoAcid::M,
                'n' => AminoAcid::N,
                'p' => AminoAcid::P,
                'q' => AminoAcid::Q,
                'r' => AminoAcid::R,
                's' => AminoAcid::S,
                't' => AminoAcid::T,
                'v' => AminoAcid::V,
                'w' => AminoAcid::W,
                'y' => AminoAcid::Y,
                invalid_char => return Err(format!("Invalid amino acid char: {}", invalid_char)),
            })
        }
        if *sequence.last().unwrap() != AminoAcid::Stop {
            sequence.push(AminoAcid::Stop)
        }
        Ok(Protein { sequence })
    }
}

impl TryFrom<&str> for Protein {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut sequence = Vec::with_capacity(s.len());

        for c in s.chars() {
            sequence.push(match c.to_ascii_lowercase() {
                'a' => AminoAcid::A,
                'c' => AminoAcid::C,
                'd' => AminoAcid::D,
                'e' => AminoAcid::E,
                'f' => AminoAcid::F,
                'g' => AminoAcid::G,
                'h' => AminoAcid::H,
                'i' => AminoAcid::I,
                'k' => AminoAcid::K,
                'l' => AminoAcid::L,
                'm' => AminoAcid::M,
                'n' => AminoAcid::N,
                'p' => AminoAcid::P,
                'q' => AminoAcid::Q,
                'r' => AminoAcid::R,
                's' => AminoAcid::S,
                't' => AminoAcid::T,
                'v' => AminoAcid::V,
                'w' => AminoAcid::W,
                'y' => AminoAcid::Y,
                invalid_char => return Err(format!("Invalid amino acid char: {}", invalid_char)),
            })
        }
        if *sequence.last().unwrap() != AminoAcid::Stop {
            sequence.push(AminoAcid::Stop)
        }
        Ok(Protein { sequence })
    }
}
