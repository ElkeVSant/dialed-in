use crate::coffee::Rating;
use crate::ui::{DraftCoffee, InputFocus};

type DraftFieldAccessor = Box<dyn Fn(&DraftCoffee) -> String>;
type InputExtractor = Option<Box<dyn Fn(&str) -> String>>;

pub enum DraftField {
    Header(&'static str),
    Field(&'static str, InputFocus, DraftFieldAccessor, InputExtractor),
    Spacing,
    Summary(&'static str, DraftFieldAccessor),
}

pub fn build_coffee_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
    let mut fields: Vec<DraftField> = vec![
        DraftField::Field(
            "Name",
            InputFocus::Name,
            Box::new(|c| c.name.clone().unwrap_or_default()),
            None,
        ),
        DraftField::Field(
            "Origin",
            InputFocus::Origin,
            Box::new(|c| c.origin.clone().unwrap_or_default()),
            None,
        ),
        DraftField::Field(
            "Varieties",
            InputFocus::Varieties,
            Box::new(|c| c.varieties.clone().unwrap_or_default()),
            Some(Box::new(|v| v.split(", ").last().unwrap_or("").to_string())),
        ),
        DraftField::Field(
            "Process",
            InputFocus::Process,
            Box::new(|c| c.process.clone().unwrap_or_default()),
            None,
        ),
        DraftField::Field(
            "Roaster",
            InputFocus::Roaster,
            Box::new(|c| c.roaster.clone().unwrap_or_default()),
            None,
        ),
        DraftField::Field(
            "Decaf",
            InputFocus::Decaf,
            Box::new(|c| {
                if c.decaf.unwrap_or(false) {
                    "☑".to_string()
                } else {
                    "☐".to_string()
                }
            }),
            None,
        ),
    ];
    if let Some(draft) = draft
        && draft.decaf == Some(true)
    {
        fields.push(DraftField::Field(
            "Decaffeination Process",
            InputFocus::DecaffeinationProcess,
            Box::new(|c: &DraftCoffee| c.decaffeination_process.clone().unwrap_or_default()),
            None,
        ));
    } else {
        fields.push(DraftField::Spacing)
    }
    fields.push(DraftField::Field(
        "Grind size",
        InputFocus::GrindSize,
        Box::new(|c| {
            c.brew_settings
                .as_ref()
                .and_then(|bs| bs.grind_size.clone())
                .unwrap_or_default()
        }),
        None,
    ));
    if let Some(draft) = draft
        && draft.brew_settings.is_some()
    {
        fields.push(DraftField::Field(
            "Adjustment",
            InputFocus::GrindSizeAdjustment,
            Box::new(|c| {
                c.brew_settings
                    .as_ref()
                    .and_then(|bs| bs.grind_size_adjustment.as_ref())
                    .map(|gsa| gsa.to_string())
                    .unwrap_or_default()
            }),
            None,
        ))
    } else {
        fields.push(DraftField::Spacing)
    }
    fields
}

pub fn build_rating_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
    let mut fields = vec![
        DraftField::Header("Aroma"),
        DraftField::Field(
            " Strength",
            InputFocus::AromaStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aroma.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Field(
            " Personal",
            InputFocus::AromaPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aroma.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Header("Sweetness"),
        DraftField::Field(
            " Strength",
            InputFocus::SweetnessStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.sweetness.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Field(
            " Personal",
            InputFocus::SweetnessPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.sweetness.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Header("Acidity"),
        DraftField::Field(
            " Strength",
            InputFocus::AcidityStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.acidity.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Field(
            " Personal",
            InputFocus::AcidityPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.acidity.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Header("Body"),
        DraftField::Field(
            " Strength",
            InputFocus::BodyStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.body.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Field(
            " Personal",
            InputFocus::BodyPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.body.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Header("Aftertaste"),
        DraftField::Field(
            " Strength",
            InputFocus::AftertasteStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aftertaste.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
        DraftField::Field(
            " Personal",
            InputFocus::AftertastePersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aftertaste.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
            None,
        ),
    ];
    if let Some(draft) = draft
        && let Some(r) = draft.rating
        && calculate_score(&r).is_some()
    {
        fields.append(&mut vec![
            DraftField::Spacing,
            DraftField::Summary(
                "Rating",
                Box::new(|c| calculate_score(&c.rating.unwrap()).unwrap().to_string()),
            ),
        ])
    } else {
        fields.append(&mut vec![DraftField::Spacing, DraftField::Spacing])
    }
    fields
}

pub fn build_notes_field() -> DraftField {
    DraftField::Field(
        "Notes",
        InputFocus::Notes,
        Box::new(|c| c.notes.clone().unwrap_or_default()),
        None,
    )
}

fn format_score(score: u8) -> String {
    format!(
        "{}{}",
        "● ".repeat(score as usize),
        "○ ".repeat((5 - score) as usize)
    )
}

pub fn calculate_score(rating: &Rating) -> Option<u8> {
    let scores = [
        rating.aroma.as_ref().and_then(|a| a.personal),
        rating.sweetness.as_ref().and_then(|s| s.personal),
        rating.acidity.as_ref().and_then(|a| a.personal),
        rating.body.as_ref().and_then(|b| b.personal),
        rating.aftertaste.as_ref().and_then(|a| a.personal),
    ];
    if scores.iter().all(|s| s.is_none()) {
        None
    } else {
        Some(scores.iter().filter_map(|s| *s).sum())
    }
}
