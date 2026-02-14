use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;

pub fn find_toml() -> Result<File, std::io::Error>  
{
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Cargo.toml")?;
    
    Ok(file)
}

pub fn add_crate(toml_file: &mut File, crate_name: &str) -> Result<(), std::io::Error>
{
    writeln!(toml_file, "{} = \"*\"", crate_name)?;
    Ok(())
}

