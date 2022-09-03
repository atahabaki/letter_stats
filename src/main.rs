use clap::Parser;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    let args = Args::parse();
    if let Ok(contents) = std::fs::read_to_string(args.file) {
        let stats = count_letters(contents.as_str());
        println!("{:#?}", stats);
    } else {
    }
}

#[derive(Debug, Default)]
struct LetterStat {
    count: u64,
    leading_stat: HashMap<char, u64>
}

fn count_letters(contents: &str) -> HashMap<char, LetterStat> {
    let mut map = HashMap::<char, LetterStat>::new();
    // read 1st char, create a hash if not exist
    let contents = contents.to_lowercase();
    let mut iter = contents.chars();
    while let Some(curr) = iter.next() {
        if !curr.is_whitespace() {
            let letter_stat = map.entry(curr).or_insert_with(LetterStat::default);
            letter_stat.count += 1;
            // then read the next char in coppied iterator, 
            // change leading_chars according to it,
            let mut iter2 = iter.clone();
            if let Some(next) = iter2.next() {
                if ! next.is_whitespace()  && next != curr {
                    letter_stat.leading_stat.entry(next).and_modify(|c| *c += 1).or_insert(1);
                }
            }
            // return back to original iterator, and so on...
        }
    }
    map
}
