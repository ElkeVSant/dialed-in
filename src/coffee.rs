use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Coffee {
    pub id: Uuid,
    pub name: String,
    pub origin: Option<String>,
    pub varieties: Option<Vec<String>>,
    pub process: Option<String>,
    pub decaf: Option<bool>,
    pub decaffeination_process: Option<String>,
    pub roaster: Option<String>,
    pub brew_settings: Option<BrewSettings>,
    pub rating: Option<Rating>,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BrewSettings {
    pub grind_size: u8,
    pub grind_size_adjustment: Option<GrindAdjustment>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GrindAdjustment {
    MuchCoarser,
    Coarser,
    Finer,
    MuchFiner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rating {
    pub aroma: Option<Score>,
    pub sweetness: Option<Score>,
    pub acidity: Option<Score>,
    pub body: Option<Score>,
    pub aftertaste: Option<Score>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub strength: u8,
    pub personal: u8,
}
