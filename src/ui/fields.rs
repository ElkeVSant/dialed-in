use crate::ui::{AddFocus, DraftCoffee};

type DraftFieldAccessor = Box<dyn Fn(&DraftCoffee) -> String>;

pub enum DraftField {
    Header(&'static str),
    Field(&'static str, AddFocus, DraftFieldAccessor),
    Spacing,
}

pub fn build_fact_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
    let mut fields: Vec<DraftField> = vec![
        DraftField::Field(
            "Name",
            AddFocus::Name,
            Box::new(|c| c.name.clone().unwrap_or_default()),
        ),
        DraftField::Field(
            "Origin",
            AddFocus::Origin,
            Box::new(|c| c.origin.clone().unwrap_or_default()),
        ),
        DraftField::Field(
            "Varieties",
            AddFocus::Varieties,
            Box::new(|c| c.varieties.clone().unwrap_or_default()),
        ),
        DraftField::Field(
            "Process",
            AddFocus::Process,
            Box::new(|c| c.process.clone().unwrap_or_default()),
        ),
        DraftField::Field(
            "Roaster",
            AddFocus::Roaster,
            Box::new(|c| c.roaster.clone().unwrap_or_default()),
        ),
        DraftField::Field(
            "Decaf",
            AddFocus::Decaf,
            Box::new(|c| {
                if c.decaf.unwrap_or(false) {
                    "☑".to_string()
                } else {
                    "☐".to_string()
                }
            }),
        ),
    ];
    if let Some(draft) = draft
        && draft.decaf == Some(true)
    {
        fields.push(DraftField::Field(
            "Decaffeination Process",
            AddFocus::DecaffeinationProcess,
            Box::new(|c: &DraftCoffee| c.decaffeination_process.clone().unwrap_or_default()),
        ));
    }
    fields
}

pub fn build_experience_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
    let mut fields: Vec<DraftField> = vec![DraftField::Field(
        "Grind size",
        AddFocus::GrindSize,
        Box::new(|c| {
            c.brew_settings
                .as_ref()
                .and_then(|bs| bs.grind_size.clone())
                .unwrap_or_default()
        }),
    )];
    if let Some(draft) = draft
        && draft.brew_settings.is_some()
    {
        fields.push(DraftField::Field(
            "Adjustment",
            AddFocus::GrindSizeAdjustment,
            Box::new(|c| {
                c.brew_settings
                    .as_ref()
                    .and_then(|bs| bs.grind_size_adjustment.as_ref())
                    .map(|gsa| gsa.to_string())
                    .unwrap_or_default()
            }),
        ))
    } else {
        fields.push(DraftField::Spacing)
    }
    let mut rating_fields = vec![
        DraftField::Header("Aroma"),
        DraftField::Field(
            " Strength",
            AddFocus::AromaStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aroma.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Field(
            " Personal",
            AddFocus::AromaPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aroma.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Header("Sweetness"),
        DraftField::Field(
            " Strength",
            AddFocus::SweetnessStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.sweetness.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Field(
            " Personal",
            AddFocus::SweetnessPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.sweetness.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Header("Acidity"),
        DraftField::Field(
            " Strength",
            AddFocus::AcidityStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.acidity.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Field(
            " Personal",
            AddFocus::AcidityPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.acidity.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Header("Body"),
        DraftField::Field(
            " Strength",
            AddFocus::BodyStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.body.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Field(
            " Personal",
            AddFocus::BodyPersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.body.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Header("Aftertaste"),
        DraftField::Field(
            " Strength",
            AddFocus::AftertasteStrength,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aftertaste.as_ref())
                        .and_then(|a| a.strength)
                        .unwrap_or(0),
                )
            }),
        ),
        DraftField::Field(
            " Personal",
            AddFocus::AftertastePersonal,
            Box::new(|c| {
                format_score(
                    c.rating
                        .as_ref()
                        .and_then(|r| r.aftertaste.as_ref())
                        .and_then(|a| a.personal)
                        .unwrap_or(0),
                )
            }),
        ),
    ];
    fields.append(&mut rating_fields);
    fields
}

fn format_score(score: u8) -> String {
    format!(
        "{}{}",
        "● ".repeat(score as usize),
        "○ ".repeat((5 - score) as usize)
    )
}
