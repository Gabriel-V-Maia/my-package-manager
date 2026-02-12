mod searcher;

use clap::Parser;
use searcher::searchCrate::CrateSearcher;

#[derive(Parser, Debug)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse();
    let searcher = CrateSearcher::new();
    
    match searcher.search_crate(&args.name) 
    {
        Ok(json) => {
            println!("Found crate: {}", args.name);
            println!("Response: {}", json);
        }
        Err(e) => {
            eprintln!("Error searching for '{}': {}", args.name, e);
        }
    }
}

