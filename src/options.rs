use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// The problem to solve.
    pub problem: String,
}
