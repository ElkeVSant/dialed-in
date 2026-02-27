use crate::coffee::{BrewSettings, Coffee, Rating, Score};

pub fn pour_coffee() -> Coffee {
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
