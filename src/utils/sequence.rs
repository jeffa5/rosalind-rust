use std::convert::TryFrom;

pub trait Sequence: TryFrom<String> {}
