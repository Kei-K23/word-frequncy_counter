use clap::Parser;
use hashbrown::HashMap as FastHashMap;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// Simple program to count word frequencies in a file

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(short, long)]
    case_insensitive: bool,

    #[clap(short, long)]
    ignore_common: bool,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.file).expect("Could not read file");
}
