mod parser;
mod searcher;

use searcher::CrateSearcher;
use clap::Parser;

fn search(crate_name: &str)
{
    let searcher = CrateSearcher::new();


    match searcher.get_crate_info(crate_name) 
    {
        Ok(info) => {
            println!("Found crate!");
            println!("{}", info.get_summary());  
        }
        Err(e) => println!("Error searching for: {} - Details: {}", crate_name, e),
    }

}

#[derive(Parser, Debug)]
struct Args
{
    #[arg(
        short = 'c', 
        long = "search",
        alias = "find",
        default_value = "nothing" 
    )]
    crate_to_search: String,   
}

fn main() {
    let search_query = Args::parse();

    search(&search_query.crate_to_search);
}

