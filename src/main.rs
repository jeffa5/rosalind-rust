extern crate clap;
extern crate itertools;
extern crate reqwest;
extern crate wl_clipboard_rs;

use clap::Parser;

use wl_clipboard_rs::copy::{MimeType, Options, Source};

mod options;
mod problems;
mod utils;

use options::Args;
use problems::*;
use utils::fasta::Fasta;
use utils::read_file;

fn make_solve<F>(f: F) -> Box<dyn Fn() -> String>
where
    F: Fn() -> String + 'static,
{
    Box::new(f)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let problems = maplit::btreemap! {
        "dna" =>  make_solve(||dna::solve(&read_file("data/dna.txt"))),
        "rna" =>  make_solve(||rna::solve(&read_file("data/rna.txt"))),
        "revc" => make_solve(||revc::solve(&read_file("data/revc.txt"))),
        "gc" => make_solve(||gc::solve(Fasta::load("data/gc.txt").unwrap())),
        "hamm" => make_solve(||hamm::solve(&read_file("data/hamm.txt"))),
        "prot" => make_solve(||prot::solve(&read_file("data/prot.txt"))),
        "prtm" => make_solve(||prtm::solve(&read_file("data/prtm.txt"))),
        "subs" => make_solve(||subs::solve(&read_file("data/subs.txt"))),
        "iprb" => make_solve(||iprb::solve(&read_file("data/iprb.txt"))),
        "fib" => make_solve(||fib::solve(&read_file("data/fib.txt"))),
        "fibd" => make_solve(||fibd::solve(&read_file("data/fibd.txt"))),
        "grph" => make_solve(||grph::solve(Fasta::load("data/grph.txt").unwrap())),
        "iev" => make_solve(||iev::solve(&read_file("data/iev.txt"))),
        "mrna" => make_solve(||mrna::solve(&read_file("data/mrna.txt"))),
        "lia" => make_solve(||lia::solve(&read_file("data/lia.txt"))),
        "cons" => make_solve(||cons::solve(Fasta::load("data/cons.txt").unwrap())),
        "prob" => make_solve(||prob::solve(&read_file("data/prob.txt"))),
        "mprt" => make_solve(||mprt::solve(&read_file("data/mprt.txt"))),
        "orf" => make_solve(||orf::solve(Fasta::load("data/orf.txt").unwrap())),
        "splc" => make_solve(||splc::solve(Fasta::load("data/splc.txt").unwrap())),
    };

    if let Some(problem) = problems.get(args.problem.as_str()) {
        let output = problem();
        println!("{}", output);

        let opts = Options::new();
        opts.copy(Source::Bytes(output.as_bytes()), MimeType::Autodetect)
            .expect("Failed to copy to clipboard");
    } else {
        eprintln!("Problem {} not matched, options are:", args.problem);
        for problem in problems.keys() {
            eprintln!("{}", problem);
        }
    }

    Ok(())
}
