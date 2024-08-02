use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show today's useless fact instead of a random one
    #[arg(short, long)]
    today: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Fact {
    id: String,
    text: String,
    source: String,
    source_url: String,
    language: String,
    permalink: String,
}

fn main() {
    let args = Args::parse();

    if args.today {
        let resp: Fact =
            match reqwest::blocking::get("https://uselessfacts.jsph.pl/api/v2/facts/today") {
                Ok(resp) => resp.json().unwrap(),
                Err(err) => panic!("Error: {}", err),
            };
        println!("Today's Random Fact\n{:#?}", resp);
    } else {
        let resp: Fact =
            match reqwest::blocking::get("https://uselessfacts.jsph.pl/api/v2/facts/random") {
                Ok(resp) => resp.json().unwrap(),
                Err(err) => panic!("Error: {}", err),
            };
        println!("Random Fact!\n{:#?}", resp);
    }
}
