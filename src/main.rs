extern crate flate2;
extern crate seq_io;
extern crate fst;

use structopt::StructOpt;
use std::fs::File;
use std::fs::OpenOptions;

use std::path::PathBuf;

use std::io::{BufReader, BufWriter, BufRead, Write};
use flate2::read::GzDecoder;

use seq_io::fastq::{Record};
use std::str;

use fst::{IntoStreamer, Set};
use fst::automaton::Levenshtein;

#[derive(StructOpt)]
struct Cli {

    /// sequence list
    #[structopt(
        short="s", 
        long = "--sequence_list",
        parse(from_os_str)
    )]
        barcode_list: std::path::PathBuf,

    /// Gzipped fastq
    #[structopt(
        short="f", 
        long = "--fastq",
        parse(from_os_str)
    )]
        fastq: std::path::PathBuf,

    /// Levenshtein Distance
    #[structopt(
        short="l", 
        long = "--levenshtein",
        default_value = "1"
    )]
    levenshtein: u32,

    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

}


fn main() {
let args = Cli::from_args();
let dist = args.levenshtein;
let gz1 = File::open(&args.fastq).expect("Could not open Fastq");
let bc = File::open(&args.barcode_list).expect("Could not open barcode list");
let bcr = BufReader::new(&bc);
let gz1 = GzDecoder::new(gz1);
let mut reader1 = seq_io::fastq::Reader::new(gz1);

let out = OpenOptions::new().create(true).write(true).append(true).open(&args.output).expect("Could not create output file");
let mut seqwriter = BufWriter::new(out);

let m_lines: Vec<String> = bcr.lines().map(|x| x.unwrap()).collect();
let sbc = m_lines.tap(|v| v.sort()); //sort barcodes before insertion into the set
let set = Set::from_iter(sbc).unwrap();
// get sequences from fastq and return seq if matches the barcode
    while let Some(record1) = reader1.next() {
        let result = record1.expect("Read Error");
        let seq1 = str::from_utf8(&result.seq()).unwrap();
        let lev1 = Levenshtein::new(seq1, dist).unwrap();
        let stream1 = set.search(lev1).into_stream();
        let keys1 = stream1.into_strs().unwrap();
        if keys1.len() >0 {
            result.write_unchanged(&mut seqwriter).expect("write error, check write permissions");
        }
    }

}

trait Tap {
    fn tap(self, f: impl FnMut(&mut Self)) -> Self;
}

impl<T> Tap for T {
    fn tap(mut self, mut f: impl FnMut(&mut Self)) -> Self {
        f(&mut self);
        self
    }
}