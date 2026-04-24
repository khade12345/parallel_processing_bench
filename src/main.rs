use clap::Parser;
use std::time::Instant;
use rayon::prelude::*;
use std::hint::black_box;
use rayon::ThreadPoolBuilder;
use rustclust::benchmark;
#[derive(Clone)]
pub struct Event {
    x: u16,
    y: u16,
    time: i64, //units are 1.5625ns (tpx manual)
    intens: u16,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Evaluates Clusters from Electron Microscopy", long_about = None)]
struct Args {

    /// Number of threads for parallel processing
    #[arg(short = 'n', long, default_value_t = 1, help_heading = "Processing Parameters")]
    n_threads: usize,

    /// Batch size in tpx3 headers (optional). Header is roughly 1kB Currently only works for tpx reading and no output hits.
    #[arg(short = 'b', long, help_heading = "Processing Parameters")]
    batch_size: Option<usize>,
 
}


fn main() {
    let args = Args::parse();
    args.batch_size;

    benchmark();
}

