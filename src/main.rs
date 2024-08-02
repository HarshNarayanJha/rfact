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

fn main() {
    let args = Args::parse();

    let endpoint = if args.today { TODAY } else { RANDOM };
    let url = format!("{}{}", FACTS_URL, endpoint);

    let resp: Fact = match reqwest::blocking::get(url) {
        Ok(resp) => match resp.json() {
            Ok(resp) => resp,
            Err(err) => panic!("Error: {}", err),
        },
        Err(err) => panic!("Error: {}", err),
    };
    println!("Today's Random Fact\n{:#?}", resp);
}
