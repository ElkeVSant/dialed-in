use std::fs;

use crate::coffee::Coffee;

fn store(coffees: Vec<Coffee>) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string(&coffees)?;
    let dir = dirs::home_dir()
        .ok_or("could not find home directory")?
        .join("Library/Application Support/Dialed In/");
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("coffee_feedback.json"), content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coffee::{BrewSettings, Rating, Score};
    #[test]
    fn test_store() {
        let coffee = Coffee {
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
        };
        assert!(store(vec![coffee]).is_ok());
    }
}
