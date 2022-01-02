use crate::utils::sequence::Sequence;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Fasta<T> {
    data: Vec<(String, T)>,
}

impl<T> Fasta<T> {
    pub fn iter(&self) -> std::slice::Iter<(String, T)> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<(String, T)> {
        self.data.iter_mut()
    }
}

impl<T: Display> std::fmt::Display for Fasta<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (id, item) in &self.data {
            writeln!(f, "{}", id)?;
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<T> TryFrom<File> for Fasta<T>
where
    T: Sequence,
    T::Error: ToString,
{
    type Error = String;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let reader = BufReader::new(file);
        let mut id = None;
        let mut sequence = String::new();
        let mut vec = Vec::new();

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => return Err(format!("{}", e)),
            };
            if line.starts_with('>') {
                if let Some(previous_id) = id {
                    let t = match T::try_from(sequence) {
                        Ok(v) => v,
                        Err(e) => return Err(e.to_string()),
                    };
                    vec.push((previous_id, t));
                    sequence = String::new();
                }
                id = Some(line[1..].to_string());
            } else {
                sequence.push_str(line.trim())
            }
        }
        if let Some(id) = id {
            let t = match T::try_from(sequence) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            vec.push((id, t));
        }
        Ok(Fasta { data: vec })
    }
}

impl<T> TryFrom<String> for Fasta<T>
where
    T: Sequence,
    T::Error: ToString,
{
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_ref())
    }
}

impl<T> TryFrom<&str> for Fasta<T>
where
    T: Sequence,
    T::Error: ToString,
{
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut id = None;
        let mut sequence = String::new();
        let mut data = Vec::new();

        for line in s.lines() {
            if line.starts_with('>') {
                if let Some(previous_id) = id {
                    let t = match T::try_from(sequence) {
                        Ok(v) => v,
                        Err(e) => return Err(e.to_string()),
                    };
                    data.push((previous_id, t));
                    sequence = String::new();
                }
                id = Some(line[1..].to_string());
            } else {
                sequence.push_str(line.trim())
            }
        }
        if let Some(id) = id {
            let t = match T::try_from(sequence) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            data.push((id, t));
        }
        Ok(Fasta { data })
    }
}
