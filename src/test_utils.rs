use crate::coffee::{BrewSettings, Coffee, Rating, Score};

pub fn pour_coffee() -> Coffee {
    Coffee {
        name: "Río Dulce".to_string(),
        origin: Some("Pijao, Quindío, Colombia".to_string()),
        varieties: Some(vec!["Gesha".to_string()]),
        process: Some("Washed".to_string()),
        decaf: Some(false),
        roaster: Some("Little Waves Coffee Roasters".to_string()),
        brew_settings: Some(BrewSettings {
            grind_size: 3,
            grind_size_adjustment: None,
        }),
        rating: Some(Rating {
            aroma: Some(Score {
                strength: 5,
                personal: 5,
            }),
            sweetness: Some(Score {
                strength: 4,
                personal: 5,
            }),
            acidity: Some(Score {
                strength: 2,
                personal: 4,
            }),
            body: Some(Score {
                strength: 4,
                personal: 5,
            }),
            aftertaste: Some(Score {
                strength: 4,
                personal: 5,
            }),
        }),
        notes: Some("Wauw! Standart; Fruit Milkshake aroma; fruity taste with acidity later in mouth, smells and tastes like it isn’t a washed process".to_string()),
        ..Default::default()
    }
}

pub fn pour_decaf() -> Coffee {
    Coffee {
        name: "Wildcard By Night".to_string(),
        origin: Some("Huila, Colombia".to_string()),
        varieties: Some(vec!["Pink Bourbon".to_string()]),
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
