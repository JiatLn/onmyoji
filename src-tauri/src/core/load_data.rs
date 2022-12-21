use anyhow::Result;
use std::{
    fs::File,
    io::{Read, Write},
};

pub fn load_data(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn save_data(path: &str, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let contents = load_data("data/account.json").unwrap();
        assert!(contents.contains("gold"));
    }
}
