use std::fs;
use std::path::Path;

use crate::coffee::Coffee;

fn store(coffees: Vec<Coffee>, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string(&coffees)?;
    fs::create_dir_all(&path)?;
    fs::write(path.join("coffee_feedback.json"), content)?;
    Ok(())
}

fn load(path: &Path) -> Result<Vec<Coffee>, Box<dyn std::error::Error>> {
    if path.exists() {
        let content = fs::read_to_string(path.join("coffee_feedback.json"))?;
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
    use crate::coffee::{BrewSettings, Rating, Score};

    fn pour_coffee() -> Coffee {
        Coffee {
            name: "Wildcard By Night".to_string(),
            origin: "Huila, Colombia".to_string(),
            variety: Some(vec!["Pink Bourbon".to_string()]),
            process: None,
            decaf: true,
            decaffeination_process: Some("Advanced".to_string()),
            roaster: "Wide Awake".to_string(),
            brew_settings: BrewSettings {
                grind_size: 4,
                grind_size_adjustment: None,
            },
            rating: Rating {
                aroma: Score {
                    strength: 5,
                    personal: 5,
                },
                sweetness: Score {
                    strength: 5,
                    personal: 5,
                },
                acidity: Score {
                    strength: 4,
                    personal: 5,
                },
                body: Score {
                    strength: 3,
                    personal: 4,
                },
                aftertaste: Score {
                    strength: 4,
                    personal: 4,
                },
            },
        }
    }

    #[test]
    fn test_store_and_load() {
        let coffee = pour_coffee();
        let temp_dir = TempDir::new().expect("could not create temp dir");
        let path = temp_dir.path();
        assert!(store(vec![coffee], path).is_ok());

        let coffees = load(path);
        let coffee = pour_coffee();
        assert_eq!(coffees.unwrap(), vec![coffee]);
    }
}
