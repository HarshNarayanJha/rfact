mod fact;
use fact::Fact;

use clap::Parser;
use reqwest;

const FACTS_URL: &str = "https://uselessfacts.jsph.pl/api/v2/facts/";
const RANDOM: &str = "random";
const TODAY: &str = "today";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Show today's useless fact instead of a random one
    #[arg(short, long)]
    today: bool,
}

fn get_fact(url: String) -> Result<Fact, reqwest::Error> {
    let resp: Result<Fact, reqwest::Error> =
        reqwest::blocking::get(url).and_then(|resp| resp.json());

    resp
}

fn main() {
    let args = Args::parse();

    let endpoint = if args.today { TODAY } else { RANDOM };
    let url = format!("{}{}", FACTS_URL, endpoint);

    let fact: Fact = match get_fact(url) {
        Ok(fact) => fact,
        Err(err) => {
            eprintln!("Error in parsing JSON: {}", err);
            std::process::exit(1);
        }
    };
    println!("{}", fact);
}
