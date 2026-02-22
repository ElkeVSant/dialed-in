pub struct Coffee {
    name: String,
    origin: String,
    variety: Vec<String>,
    process: String,
    decaf: bool,
    decaffeination_process: Option<String>,
    roaster: String,
    brew_settings: BrewSettings,
    rating: Rating,
}

pub struct BrewSettings {
    grind_size: u8,
    grind_size_adjustment: Option<GrindAdjustment>,
}

pub enum GrindAdjustment {
    MuchCoarser,
    Coarser,
    Finer,
    MuchFiner,
}

pub struct Rating {
    aroma: Score,
    sweetness: Score,
    acidity: Score,
    body: Score,
    aftertaste: Score,
}

pub struct Score {
    strength: u8,
    personal: u8,
}
