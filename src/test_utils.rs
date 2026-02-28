use crate::coffee::{BrewSettings, Coffee, Rating, Score};

pub fn pour_coffee() -> Coffee {
    Coffee {
        name: "Wildcard By Night".to_string(),
        origin: Some("Huila, Colombia".to_string()),
        variety: Some(vec!["Pink Bourbon".to_string()]),
        decaf: Some(true),
        decaffeination_process: Some("Advanced".to_string()),
        roaster: Some("Wide Awake".to_string()),
        brew_settings: Some(BrewSettings {
            grind_size: 4,
            grind_size_adjustment: None,
        }),
        rating: Some(Rating {
            aroma: Some(Score {
                strength: 5,
                personal: 5,
            }),
            sweetness: Some(Score {
                strength: 5,
                personal: 5,
            }),
            acidity: Some(Score {
                strength: 4,
                personal: 5,
            }),
            body: Some(Score {
                strength: 3,
                personal: 4,
            }),
            aftertaste: Some(Score {
                strength: 4,
                personal: 4,
            }),
        }),
        ..Default::default()
    }
}
