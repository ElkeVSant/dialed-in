use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Coffee {
    pub name: String,
    pub origin: String,
    pub variety: Option<Vec<String>>,
    pub process: Option<String>,
    pub decaf: bool,
    pub decaffeination_process: Option<String>,
    pub roaster: String,
    pub brew_settings: BrewSettings,
    pub rating: Rating,
}

#[derive(Serialize, Deserialize)]
pub struct BrewSettings {
    pub grind_size: u8,
    pub grind_size_adjustment: Option<GrindAdjustment>,
}

#[derive(Serialize, Deserialize)]
pub enum GrindAdjustment {
    MuchCoarser,
    Coarser,
    Finer,
    MuchFiner,
}

#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub aroma: Score,
    pub sweetness: Score,
    pub acidity: Score,
    pub body: Score,
    pub aftertaste: Score,
}

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub strength: u8,
    pub personal: u8,
}
