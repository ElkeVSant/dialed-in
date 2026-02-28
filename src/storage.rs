use std::fs;
use std::path::Path;

use crate::coffee::Coffee;

pub fn store(coffees: Vec<Coffee>, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string(&coffees)?;
    fs::create_dir_all(&path)?;
    fs::write(path.join("coffee_feedback.json"), content)?;
    Ok(())
}

pub fn load(path: &Path) -> Result<Vec<Coffee>, Box<dyn std::error::Error>> {
    let file_path = path.join("coffee_feedback.json");
    if file_path.exists() {
        let content = fs::read_to_string(file_path)?;
        let coffees: Vec<Coffee> = serde_json::from_str(&content)?;
        Ok(coffees)
    } else {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::test_utils::pour_coffee;

    #[test]
    fn test_store_and_load() {
        let stored_coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();
        let id = stored_coffee.id;
        assert!(store(vec![stored_coffee], path).is_ok());

        let coffees = load(path);
        let mut loaded_coffee = pour_coffee();
        loaded_coffee.id = id;
        assert_eq!(coffees.expect("no beans in grinder"), vec![loaded_coffee]);
    }
}
