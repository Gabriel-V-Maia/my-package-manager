mod parser;
mod searcher;
mod manager;

use searcher::CrateSearcher;
use clap::Parser;

fn search(crate_name: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let searcher = CrateSearcher::new();
    
    let info = searcher.get_crate_info(crate_name)?;
    println!("Found crate!");
    println!("{}", info.get_summary());
    
    let mut toml_file = manager::manager::find_toml()?;
    manager::manager::add_crate(&mut toml_file, crate_name)?;
    
    println!("Crate '{}' added to Cargo.toml!", crate_name);
    Ok(())
}
#[derive(Parser, Debug)]
struct Args
{
    #[arg(
        short = 's', 
        long = "search",
        alias = "find",
        default_value = "nothing" 
    )]
    crate_to_search: String,   
}

fn main() {
    let search_query = Args::parse();

    if let Err(e) = search(&search_query.crate_to_search) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }


}

