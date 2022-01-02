extern crate clap;
extern crate itertools;
extern crate reqwest;
extern crate wl_clipboard_rs;

use clap::{App, Arg};

use wl_clipboard_rs::copy::{MimeType, Options, Source};

use std::convert::TryFrom;
use std::fs::File;

mod problems;
mod utils;

use problems::*;
use utils::fasta::Fasta;
use utils::read_file;

fn main() {
    let matches = App::new("rosalind")
        .about("Solutions for rosalind problems")
        .author("Jeffas")
        .arg(
            Arg::with_name("PROBLEM")
                .help("The problem to solve")
                .required(true)
                .index(1),
        )
        .get_matches();

    let problem = matches.value_of("PROBLEM").unwrap();

    let output = match problem.to_ascii_lowercase().as_ref() {
        "dna" => dna::solve(&read_file("data/dna.txt")),
        "rna" => rna::solve(&read_file("data/rna.txt")),
        "revc" => revc::solve(&read_file("data/revc.txt")),
        "gc" => gc::solve(Fasta::try_from(File::open("data/gc.txt").unwrap()).unwrap()),
        "hamm" => hamm::solve(&read_file("data/hamm.txt")),
        "prot" => prot::solve(&read_file("data/prot.txt")),
        "prtm" => prtm::solve(&read_file("data/prtm.txt")),
        "subs" => subs::solve(&read_file("data/subs.txt")),
        "iprb" => iprb::solve(&read_file("data/iprb.txt")),
        "fib" => fib::solve(&read_file("data/fib.txt")),
        "fibd" => fibd::solve(&read_file("data/fibd.txt")),
        "grph" => grph::solve(Fasta::try_from(File::open("data/grph.txt").unwrap()).unwrap()),
        "iev" => iev::solve(&read_file("data/iev.txt")),
        "mrna" => mrna::solve(&read_file("data/mrna.txt")),
        "lia" => lia::solve(&read_file("data/lia.txt")),
        "cons" => cons::solve(Fasta::try_from(File::open("data/cons.txt").unwrap()).unwrap()),
        "prob" => prob::solve(&read_file("data/prob.txt")),
        "mprt" => mprt::solve(&read_file("data/mprt.txt")),
        "orf" => orf::solve(Fasta::try_from(File::open("data/orf.txt").unwrap()).unwrap()),
        "splc" => splc::solve(Fasta::try_from(File::open("data/splc.txt").unwrap()).unwrap()),
        _ => {
            eprintln!("Problem {} not matched", problem);
            "".into()
        }
    };

    println!("{}", output);

    let opts = Options::new();
    opts.copy(Source::Bytes(output.as_bytes()), MimeType::Autodetect)
        .expect("Failed to copy to clipboard");
}
