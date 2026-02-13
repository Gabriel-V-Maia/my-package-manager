
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponse {
    #[serde(rename = "crate")]  // ← Aspas!
    pub crate_info: CrateInfo,   // ← snake_case
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrateInfo { 
    pub name: String,
    #[serde(rename = "max_version")]  
    pub version: String,  
    pub description: Option<String>, 
    pub homepage: Option<String>,   
    pub documentation: Option<String>,
}

impl CrateInfo {
    pub fn get_summary(&self) -> String {
        format!(
            "{} - {}\n{}\n{}\n{}",
            self.name,
            self.version,
            self.description.as_ref().unwrap_or(&"No description".to_string()),
            self.homepage.as_ref().unwrap_or(&"No homepage".to_string()),
            self.documentation.as_ref().unwrap_or(&"No docs".to_string())
        )
    }
    
    pub fn has_documentation(&self) -> bool {
        self.documentation.is_some()  // ← minúscula!
    }
}

impl JsonResponse {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
    
    pub fn get_crate_info(&self) -> &CrateInfo {
        &self.crate_info
    }
}

