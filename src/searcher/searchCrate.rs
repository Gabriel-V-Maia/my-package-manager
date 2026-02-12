use reqwest;

pub struct CrateSearcher 
{
    base_url: String,
    client: reqwest::blocking::Client,
}

impl CrateSearcher 
{
    pub fn new() -> Self 
    {
        CrateSearcher {
            base_url: "https://crates.io/api/v1/crates".to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }
    
    fn get_url(&self, crate_name: &str) -> String 
    {
        format!("{}/{}", self.base_url, crate_name)
    }
    
    pub fn search_crate(&self, crate_name: &str) -> Result<String, Box<dyn std::error::Error>>
    {
        let url = self.get_url(crate_name);
        let body = self.client
            .get(&url)
            .header("User-Agent", "my-package-manager")
            .send()?
            .text()?;
        Ok(body)
    }
}

