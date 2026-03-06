use std::fmt::{Display, Formatter, Result};

use chrono::{DateTime, Utc};
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrewSettings {
    pub grind_size: f32,
    pub grind_size_adjustment: Option<GrindAdjustment>,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum GrindAdjustment {
    MuchCoarser,
    Coarser,
    Finer,
    MuchFiner,
}

impl Display for GrindAdjustment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GrindAdjustment::MuchCoarser => write!(f, "Much coarser"),
            GrindAdjustment::Coarser => write!(f, "Coarser"),
            GrindAdjustment::Finer => write!(f, "Finer"),
            GrindAdjustment::MuchFiner => write!(f, "Much Finer"),
        }
    }
}

impl GrindAdjustment {
    pub fn coarser(&self) -> Option<GrindAdjustment> {
        match self {
            GrindAdjustment::MuchCoarser => Some(GrindAdjustment::MuchCoarser),
            GrindAdjustment::Coarser => Some(GrindAdjustment::MuchCoarser),
            GrindAdjustment::Finer => None,
            GrindAdjustment::MuchFiner => Some(GrindAdjustment::Finer),
        }
    }
    pub fn finer(&self) -> Option<GrindAdjustment> {
        match self {
            GrindAdjustment::MuchCoarser => Some(GrindAdjustment::Coarser),
            GrindAdjustment::Coarser => None,
            GrindAdjustment::Finer => Some(GrindAdjustment::MuchFiner),
            GrindAdjustment::MuchFiner => Some(GrindAdjustment::MuchFiner),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rating {
    pub aroma: Option<Score>,
    pub sweetness: Option<Score>,
    pub acidity: Option<Score>,
    pub body: Option<Score>,
    pub aftertaste: Option<Score>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub strength: Option<u8>,
    pub personal: Option<u8>,
}
