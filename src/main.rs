use clap::Parser;
use hashbrown::HashMap as FastHashMap;
use regex::Regex;
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

    let word_counts = count_words(&content, args.case_insensitive, args.ignore_common);

    println!();
    println!("Word Frequencies Counter CLI");
    println!();
    for (word, count) in word_counts {
        println!("{} : {}", word, count);
    }
}

fn count_words(
    content: &str,
    case_insensitive: bool,
    ignore_common: bool,
) -> FastHashMap<String, usize> {
    let mut word_counts: FastHashMap<String, usize> = FastHashMap::new();

    // Define common words to ignore
    let common_words = if ignore_common {
        Some(
            ["the", "and", "in", "on", "at", "of", "a", "is", "it", "to"]
                .iter()
                .map(|&word| (word, ())) // Convert &str to (&str, ())
                .collect::<FastHashMap<&str, ()>>(),
        )
    } else {
        None
    };

    let re = Regex::new(r"\w+").unwrap();
    for word in re.find_iter(content) {
        let mut word = word.as_str().to_string();

        if case_insensitive {
            word = word.to_lowercase();
        }

        if let Some(ref common_words) = common_words {
            if common_words.contains_key(word.as_str()) {
                continue;
            }
        }

        *word_counts.entry(word).or_insert(0) += 1;
    }

    word_counts
}
