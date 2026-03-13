use ratatui::crossterm::event::KeyCode;
use uuid::Uuid;

use crate::coffee::{BrewSettings, Coffee, GrindAdjustment, Rating, Score};
use crate::ui::InputFocus;

#[derive(Default)]
pub(super) struct DraftCoffee {
    pub(super) id: Option<Uuid>,
    pub(super) name: Option<String>,
    pub(super) origin: Option<String>,
    pub(super) varieties: Option<String>,
    pub(super) process: Option<String>,
    pub(super) decaf: Option<bool>,
    pub(super) decaffeination_process: Option<String>,
    pub(super) roaster: Option<String>,
    pub(super) brew_settings: Option<DraftBrewSettings>,
    pub(super) rating: Option<Rating>,
    pub(super) notes: Option<String>,
}

#[derive(Default)]
pub(super) struct DraftBrewSettings {
    pub(super) grind_size: Option<String>,
    pub(super) grind_size_adjustment: Option<GrindAdjustment>,
}

impl DraftCoffee {
    pub(super) fn toggle_decaf(&mut self) {
        self.decaf = Some(!self.decaf.unwrap_or_default());
    }
    pub(super) fn grind_coarser(&mut self) {
        let bs = self.brew_settings.as_mut().expect("no brew settings exist");
        bs.grind_size_adjustment = match bs.grind_size_adjustment {
            Some(gsa) => gsa.coarser(),
            None => Some(GrindAdjustment::Coarser),
        };
    }
    pub(super) fn grind_finer(&mut self) {
        let bs = self.brew_settings.as_mut().expect("no brew settings exist");
        bs.grind_size_adjustment = match bs.grind_size_adjustment {
            Some(gsa) => gsa.finer(),
            None => Some(GrindAdjustment::Finer),
        };
    }
    pub(super) fn reset_grind_adjustment(&mut self) {
        self.brew_settings
            .as_mut()
            .expect("no brew settings exist")
            .grind_size_adjustment = None;
    }
}

pub(super) fn update_draft_coffee(
    coffee: &mut Option<DraftCoffee>,
    focus: &InputFocus,
    keycode: KeyCode,
) {
    let coffee = coffee.get_or_insert_with(DraftCoffee::default);
    match keycode {
        KeyCode::Char(c) => match focus {
            InputFocus::Name => coffee.name.get_or_insert_with(String::new).push(c),
            InputFocus::Origin => coffee.origin.get_or_insert_with(String::new).push(c),
            InputFocus::Varieties => coffee.varieties.get_or_insert_with(String::new).push(c),
            InputFocus::Process => coffee.process.get_or_insert_with(String::new).push(c),
            InputFocus::Roaster => coffee.roaster.get_or_insert_with(String::new).push(c),
            // 5 (decaf) is handled in the event loop (Enter branch)
            InputFocus::DecaffeinationProcess => coffee
                .decaffeination_process
                .get_or_insert_with(String::new)
                .push(c),
            InputFocus::GrindSize => coffee
                .brew_settings
                .get_or_insert_with(DraftBrewSettings::default)
                .grind_size
                .get_or_insert_with(String::new)
                .push(c),
            InputFocus::AromaStrength => {
                let aroma = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aroma
                    .get_or_insert_with(Score::default);
                set_score(&mut aroma.strength, c);
            }
            InputFocus::AromaPersonal => {
                let aroma = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aroma
                    .get_or_insert_with(Score::default);
                set_score(&mut aroma.personal, c);
            }
            InputFocus::SweetnessStrength => {
                let sweetness = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .sweetness
                    .get_or_insert_with(Score::default);
                set_score(&mut sweetness.strength, c);
            }
            InputFocus::SweetnessPersonal => {
                let sweetness = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .sweetness
                    .get_or_insert_with(Score::default);
                set_score(&mut sweetness.personal, c);
            }
            InputFocus::AcidityStrength => {
                let acidity = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .acidity
                    .get_or_insert_with(Score::default);
                set_score(&mut acidity.strength, c);
            }
            InputFocus::AcidityPersonal => {
                let acidity = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .acidity
                    .get_or_insert_with(Score::default);
                set_score(&mut acidity.personal, c);
            }
            InputFocus::BodyStrength => {
                let body = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .body
                    .get_or_insert_with(Score::default);
                set_score(&mut body.strength, c);
            }
            InputFocus::BodyPersonal => {
                let body = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .body
                    .get_or_insert_with(Score::default);
                set_score(&mut body.personal, c);
            }
            InputFocus::AftertasteStrength => {
                let aftertaste = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aftertaste
                    .get_or_insert_with(Score::default);
                set_score(&mut aftertaste.strength, c);
            }
            InputFocus::AftertastePersonal => {
                let aftertaste = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aftertaste
                    .get_or_insert_with(Score::default);
                set_score(&mut aftertaste.personal, c);
            }
            InputFocus::Notes => coffee.notes.get_or_insert_with(String::new).push(c),
            _ => (),
        },
        KeyCode::Backspace => match focus {
            InputFocus::Name => pop_optional_char(&mut coffee.name),
            InputFocus::Origin => pop_optional_char(&mut coffee.origin),
            InputFocus::Varieties => pop_optional_char(&mut coffee.varieties),
            InputFocus::Process => pop_optional_char(&mut coffee.process),
            InputFocus::Roaster => pop_optional_char(&mut coffee.roaster),
            // 5 (decaf) is handled in the event loop (Enter branch)
            InputFocus::DecaffeinationProcess => {
                pop_optional_char(&mut coffee.decaffeination_process)
            }
            InputFocus::GrindSize => {
                let dbs = coffee
                    .brew_settings
                    .get_or_insert_with(DraftBrewSettings::default);
                pop_optional_char(&mut dbs.grind_size);
                if dbs.grind_size.as_deref() == Some("") {
                    coffee.brew_settings = None;
                }
            }
            InputFocus::AromaStrength => {
                if let Some(aroma) = coffee.rating.as_mut().and_then(|r| r.aroma.as_mut()) {
                    aroma.strength = None;
                }
            }
            InputFocus::AromaPersonal => {
                if let Some(aroma) = coffee.rating.as_mut().and_then(|r| r.aroma.as_mut()) {
                    aroma.personal = None;
                }
            }
            InputFocus::SweetnessStrength => {
                if let Some(sweetness) = coffee.rating.as_mut().and_then(|r| r.sweetness.as_mut()) {
                    sweetness.strength = None;
                }
            }
            InputFocus::SweetnessPersonal => {
                if let Some(sweetness) = coffee.rating.as_mut().and_then(|r| r.sweetness.as_mut()) {
                    sweetness.personal = None;
                }
            }
            InputFocus::AcidityStrength => {
                if let Some(acidity) = coffee.rating.as_mut().and_then(|r| r.acidity.as_mut()) {
                    acidity.strength = None;
                }
            }
            InputFocus::AcidityPersonal => {
                if let Some(acidity) = coffee.rating.as_mut().and_then(|r| r.acidity.as_mut()) {
                    acidity.personal = None;
                }
            }
            InputFocus::BodyStrength => {
                if let Some(body) = coffee.rating.as_mut().and_then(|r| r.body.as_mut()) {
                    body.strength = None;
                }
            }
            InputFocus::BodyPersonal => {
                if let Some(body) = coffee.rating.as_mut().and_then(|r| r.body.as_mut()) {
                    body.personal = None;
                }
            }
            InputFocus::AftertasteStrength => {
                if let Some(aftertaste) = coffee.rating.as_mut().and_then(|r| r.aftertaste.as_mut())
                {
                    aftertaste.strength = None;
                }
            }
            InputFocus::AftertastePersonal => {
                if let Some(aftertaste) = coffee.rating.as_mut().and_then(|r| r.aftertaste.as_mut())
                {
                    aftertaste.personal = None;
                }
            }
            InputFocus::Notes => pop_optional_char(&mut coffee.notes),
            _ => (),
        },
        _ => (),
    }
}

pub(super) fn pop_optional_char(field: &mut Option<String>) {
    if field.is_some() {
        if field.as_ref().unwrap().len() == 1 {
            *field = None;
        } else {
            field.as_mut().unwrap().pop();
        }
    }
}

pub(super) fn accept_suggestion(coffee: &mut DraftCoffee, focus: &InputFocus, suggestion: String) {
    match focus {
        InputFocus::Origin => coffee.origin = Some(suggestion),
        InputFocus::Varieties => {
            let mut vars: Vec<&str> = coffee
                .varieties
                .as_deref()
                .unwrap_or_default()
                .split(", ")
                .collect();
            vars.pop();
            coffee.varieties = Some(vars.join(", ") + &suggestion);
        }
        InputFocus::Process => coffee.process = Some(suggestion),
        InputFocus::Roaster => coffee.roaster = Some(suggestion),
        InputFocus::DecaffeinationProcess => coffee.decaffeination_process = Some(suggestion),
        _ => (),
    }
}

fn set_score(score: &mut Option<u8>, value: char) {
    match value {
        '+' => *score = Some((score.unwrap_or(0) + 1).min(5)),
        '-' => *score = Some(score.unwrap_or(0).saturating_sub(1)),
        _ => {
            *score = value.to_digit(10).map(|d| d as u8).or(*score).min(Some(5));
        }
    }
}

pub(super) fn convert_draft_to_coffee(
    draft: &DraftCoffee,
) -> Result<Coffee, Box<dyn std::error::Error>> {
    Ok(Coffee {
        id: draft.id.unwrap_or_default(),
        name: draft.name.clone().ok_or("name is required")?,
        origin: draft.origin.clone(),
        varieties: draft
            .varieties
            .clone()
            .map(|v| v.split(", ").map(|s| s.to_string()).collect()),
        process: draft.process.clone(),
        roaster: draft.roaster.clone(),
        decaf: draft.decaf,
        decaffeination_process: draft.decaffeination_process.clone(),
        brew_settings: draft
            .brew_settings
            .as_ref()
            .map(|bs| -> Result<BrewSettings, Box<dyn std::error::Error>> {
                Ok(BrewSettings {
                    grind_size: bs
                        .grind_size
                        .as_deref()
                        .expect("brew settings without grind size")
                        .parse::<f32>()
                        .map_err(|_| "invalid grind size")?,
                    grind_size_adjustment: bs.grind_size_adjustment,
                })
            })
            .transpose()?,
        rating: draft.rating.as_ref().map(|r| Rating {
            aroma: r.aroma,
            sweetness: r.sweetness,
            acidity: r.acidity,
            body: r.body,
            aftertaste: r.aftertaste,
        }),
        notes: draft.notes.clone(),
        ..Default::default()
    })
}

pub(super) fn convert_coffee_to_draft(coffee: &Coffee) -> DraftCoffee {
    DraftCoffee {
        id: Some(coffee.id),
        name: Some(coffee.name.clone()),
        origin: coffee.origin.clone(),
        varieties: Some(coffee.varieties.clone().unwrap_or_default().join(", ")),
        process: coffee.process.clone(),
        roaster: coffee.roaster.clone(),
        decaf: coffee.decaf,
        decaffeination_process: coffee.decaffeination_process.clone(),
        brew_settings: coffee.brew_settings.map(|bs| DraftBrewSettings {
            grind_size: Some(bs.grind_size.to_string()),
            grind_size_adjustment: bs.grind_size_adjustment,
        }),
        rating: coffee.rating,
        notes: coffee.notes.clone(),
    }
}

// supports only the characteristics with suggestions
pub(super) fn get_match_input(focus: &InputFocus, coffee: &DraftCoffee) -> Option<String> {
    match focus {
        InputFocus::Origin => coffee.origin.clone(),
        InputFocus::Varieties => {
            if let Some(v) = &coffee.varieties {
                v.split(", ").last().map(|s| s.to_string())
            } else {
                None
            }
        }
        InputFocus::Process => coffee.process.clone(),
        InputFocus::Roaster => coffee.roaster.clone(),
        InputFocus::DecaffeinationProcess => coffee.decaffeination_process.clone(),
        _ => None,
    }
}
