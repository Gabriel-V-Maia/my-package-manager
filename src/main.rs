mod parser;
mod searcher;

use searcher::CrateSearcher;

fn main() {
    let searcher = CrateSearcher::new();
    
    match searcher.get_crate_info("serde") {
        Ok(info) => {
            println!("Found crate!");
            println!("{}", info.get_summary());  
        }
        Err(e) => println!("Error: {}", e),
    }
}

