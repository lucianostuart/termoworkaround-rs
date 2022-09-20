use clap::Parser;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

/// Rust powered CLI tool to help on games like Wordle and Termo :)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short = 'c', long, value_parser)]
    correct_words: String,

    /// Number of times to greet
    #[clap(short = 'w', long, value_parser)]
    wrong_words: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let all_words = get_all_words("./lib/5_letters.txt");

    //println!("{}", all_words);

    // TODO: make regex for search words that match

    let re = Regex::new(r"na").unwrap(); // TODO: make regex

    println!("{:?}", re.find(&all_words));
}

fn get_all_words(path: &str) -> String {
    let mut str = String::new();
    let file = File::open(path).expect("Error in reading file");
    let mut buffer_reader = BufReader::new(file);
    buffer_reader
        .read_to_string(&mut str)
        .expect("Unable to read line");

    return str;
}
