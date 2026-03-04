use ratatui::{
    crossterm::event::{Event, KeyCode, read},
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
    {DefaultTerminal, Frame},
};

use crate::app::{add_coffee, delete_coffee, list_coffees};
use crate::coffee::{BrewSettings, Coffee, GrindAdjustment, Rating, Score};

const DIALED_IN: &str = r#"
8888888b. d8b        888             8888888888         
888  "Y88bY8P        888             888  888           
888    888           888             888  888           
888    888888 8888b. 888 .d88b.  .d88888  888  88888b.  
888    888888    "88b888d8P  Y8bd88" 888  888  888 "88b 
888    888888.d88888888888888888888  888  888  888  888 
888  .d88P888888  888888Y8b.    Y88b 888  888  888  888 
8888888P" 888"Y888888888 "Y8888  "Y888888888888888  888 "#;

type DraftFieldAccessor = Box<dyn Fn(&DraftCoffee) -> String>;

enum DraftField {
    Header(&'static str),
    Field(&'static str, AddFocus, DraftFieldAccessor),
    Spacing,
}

#[derive(Default)]
struct State {
    coffees: Vec<Coffee>,
    ui_state: UiState,
}

#[derive(Default)]
struct UiState {
    mode: Mode,
    add_focus: AddFocus,
    list_state: ListState,
    coffee: Option<DraftCoffee>,
    error: Option<String>,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Normal,
    Add,
    Delete,
}

#[derive(Default, PartialEq)]
enum AddFocus {
    #[default]
    Name,
    Origin,
    Varieties,
    Process,
    Roaster,
    Decaf,
    DecaffeinationProcess,
    GrindSize,
    GrindSizeAdjustment,
    AromaStrength,
    AromaPersonal,
    SweetnessStrength,
    SweetnessPersonal,
    AcidityStrength,
    AcidityPersonal,
    BodyStrength,
    BodyPersonal,
    AftertasteStrength,
    AftertastePersonal,
}

impl AddFocus {
    fn next(&self, coffee: &Option<DraftCoffee>) -> AddFocus {
        match self {
            AddFocus::Name => AddFocus::Origin,
            AddFocus::Origin => AddFocus::Varieties,
            AddFocus::Varieties => AddFocus::Process,
            AddFocus::Process => AddFocus::Roaster,
            AddFocus::Roaster => AddFocus::Decaf,
            AddFocus::Decaf => {
                if let Some(coffee) = coffee
                    && coffee.decaf.unwrap_or_default()
                {
                    AddFocus::DecaffeinationProcess
                } else {
                    AddFocus::GrindSize
                }
            }
            AddFocus::DecaffeinationProcess => AddFocus::GrindSize,
            AddFocus::GrindSize => {
                if let Some(coffee) = coffee
                    && coffee.brew_settings.is_some()
                {
                    AddFocus::GrindSizeAdjustment
                } else {
                    AddFocus::GrindSizeAdjustment.next(coffee)
                }
            }
            AddFocus::GrindSizeAdjustment => AddFocus::AromaStrength,
            AddFocus::AromaStrength => AddFocus::AromaPersonal,
            AddFocus::AromaPersonal => AddFocus::SweetnessStrength,
            AddFocus::SweetnessStrength => AddFocus::SweetnessPersonal,
            AddFocus::SweetnessPersonal => AddFocus::AcidityStrength,
            AddFocus::AcidityStrength => AddFocus::AcidityPersonal,
            AddFocus::AcidityPersonal => AddFocus::BodyStrength,
            AddFocus::BodyStrength => AddFocus::BodyPersonal,
            AddFocus::BodyPersonal => AddFocus::AftertasteStrength,
            AddFocus::AftertasteStrength => AddFocus::AftertastePersonal,
            AddFocus::AftertastePersonal => AddFocus::Name,
        }
    }
}

#[derive(Default)]
pub struct DraftCoffee {
    pub name: Option<String>,
    pub origin: Option<String>,
    pub varieties: Option<String>,
    pub process: Option<String>,
    pub decaf: Option<bool>,
    pub decaffeination_process: Option<String>,
    pub roaster: Option<String>,
    pub brew_settings: Option<DraftBrewSettings>,
    pub rating: Option<Rating>,
}

#[derive(Default)]
pub struct DraftBrewSettings {
    pub grind_size: Option<String>,
    pub grind_size_adjustment: Option<GrindAdjustment>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let path = dirs::data_dir()
        .expect("could not find data directory")
        .join("Dialed In");

    let mut state = State {
        coffees: list_coffees(&path).expect("could not list coffees"),
        ..Default::default()
    };

    loop {
        terminal.draw(|frame| {
            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(10), Constraint::Min(0)])
                .split(frame.area());

            render_app_name(frame, areas[0]);
            render_coffees(
                &mut state.ui_state.list_state,
                &state.coffees,
                frame,
                areas[1],
            );

            if state.ui_state.mode == Mode::Normal {
                if let Some(message) = &state.ui_state.error {
                    render_error(message, frame, areas[1]);
                }
            } else if state.ui_state.mode == Mode::Add {
                render_add_coffee_modal(
                    &state.ui_state.add_focus,
                    &state.ui_state.coffee,
                    &state.ui_state.error,
                    frame,
                    areas[1],
                );
            } else if state.ui_state.mode == Mode::Delete {
                render_delete_coffee_modal(
                    &state.coffees[state.ui_state.list_state.selected().expect("no selection")],
                    frame,
                    areas[1],
                );
            }
        })?;

        if let Event::Key(key) = read()? {
            match state.ui_state.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Esc => break Ok(()),
                    KeyCode::Tab => {
                        state.ui_state.error = None;
                        if let Some(index) = state.ui_state.list_state.selected()
                            && index == state.coffees.len() - 1
                        {
                            state.ui_state.list_state.select_first();
                        } else {
                            state.ui_state.list_state.select_next();
                        }
                    }
                    KeyCode::Char('a') => {
                        state.ui_state.mode = Mode::Add;
                        state.ui_state.add_focus = AddFocus::default();
                        state.ui_state.error = None;
                    }
                    KeyCode::Char('d') => {
                        if state.ui_state.list_state.selected().is_none() {
                            state.ui_state.error = Some("Select a coffee to delete it".to_string());
                        } else {
                            state.ui_state.mode = Mode::Delete;
                        }
                    }
                    _ => (),
                },
                Mode::Add => match key.code {
                    KeyCode::Tab => {
                        state.ui_state.add_focus =
                            state.ui_state.add_focus.next(&state.ui_state.coffee)
                    }
                    KeyCode::Enter => {
                        if state.ui_state.add_focus == AddFocus::Decaf {
                            let draft = state
                                .ui_state
                                .coffee
                                .get_or_insert_with(DraftCoffee::default);
                            draft.decaf = Some(!draft.decaf.unwrap_or_default());
                        } else if let Some(draft) = &state.ui_state.coffee {
                            match convert_draft_to_coffee(draft) {
                                Ok(coffee) => {
                                    add_coffee(coffee, &path).expect("cannot remember coffee");
                                    state.coffees =
                                        list_coffees(&path).expect("could not list coffees");
                                    state.ui_state.coffee = None;
                                    state.ui_state.list_state = ListState::default();
                                    state.ui_state.mode = Mode::Normal;
                                }
                                Err(e) => state.ui_state.error = Some(e.to_string()),
                            }
                        }
                    }
                    KeyCode::Esc => {
                        state.ui_state.coffee = None;
                        state.ui_state.mode = Mode::Normal;
                    }
                    KeyCode::Backspace | KeyCode::Char(_) => {
                        if state.ui_state.add_focus == AddFocus::GrindSizeAdjustment {
                            let bs = state
                                .ui_state
                                .coffee
                                .get_or_insert_with(DraftCoffee::default)
                                .brew_settings
                                .as_mut()
                                .expect("no brew settings exist");
                            if key.code == KeyCode::Char('+') {
                                bs.grind_size_adjustment = match &bs.grind_size_adjustment {
                                    Some(gsa) => gsa.coarser(),
                                    None => Some(GrindAdjustment::Coarser),
                                }
                            } else if key.code == KeyCode::Char('-') {
                                bs.grind_size_adjustment = match &bs.grind_size_adjustment {
                                    Some(gsa) => gsa.finer(),
                                    None => Some(GrindAdjustment::Finer),
                                }
                            }
                        } else {
                            update_draft_coffee(
                                &mut state.ui_state.coffee,
                                &state.ui_state.add_focus,
                                key.code,
                            );
                            if matches!(key.code, KeyCode::Char(_)) {
                                state.ui_state.error = None;
                            }
                        }
                    }
                    _ => (),
                },
                Mode::Delete => match key.code {
                    KeyCode::Enter => {
                        delete_coffee(
                            state.coffees[state
                                .ui_state
                                .list_state
                                .selected()
                                .expect("dropped all beans")]
                            .id,
                            &path,
                        )
                        .expect("cannot purge grinder");
                        state.coffees = list_coffees(&path).expect("could not list coffees");
                        state.ui_state.list_state = ListState::default();
                        state.ui_state.mode = Mode::Normal;
                    }
                    KeyCode::Esc => state.ui_state.mode = Mode::Normal,
                    KeyCode::Char('q') => break Ok(()),
                    _ => (),
                },
            }
        }
    }
}

fn render_app_name(frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new(DIALED_IN), area);
}

fn render_coffees(state: &mut ListState, coffees: &[Coffee], frame: &mut Frame, area: Rect) {
    let coffee_names: Vec<ListItem> = coffees
        .iter()
        .map(|c| ListItem::new(c.name.clone()))
        .collect();
    let coffee_list = List::new(coffee_names).highlight_style(Style::new().bg(Color::DarkGray));
    frame.render_stateful_widget(coffee_list, area, state);
}

fn render_add_coffee_modal(
    focus: &AddFocus,
    coffee: &Option<DraftCoffee>,
    error: &Option<String>,
    frame: &mut Frame,
    area: Rect,
) {
    let modal_area = area.inner(Margin::new(2, 1));
    let modal = Block::bordered().title("New coffee");
    let inner_modal_area = modal.inner(modal_area);
    frame.render_widget(Clear, modal_area);
    frame.render_widget(modal, modal_area);

    let fact_fields = build_fact_fields(coffee);
    let experience_fields = build_experience_fields(coffee);

    let inner_modal_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(
            fact_fields.len().max(experience_fields.len()) as u16,
        )])
        .split(inner_modal_area);
    let field_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(inner_modal_areas[0]);
    let fact_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1);
            fact_fields.len().max(experience_fields.len())
        ])
        .split(field_areas[0]);
    let experience_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1);
            fact_fields.len().max(experience_fields.len())
        ])
        .split(field_areas[1]);

    fact_fields
        .iter()
        .zip(fact_areas.iter())
        .for_each(|(field, area)| {
            if let DraftField::Field(label, focus_name, value_accessor) = field {
                let fact_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(label.len() as u16 + 2),
                        Constraint::Min(0),
                    ])
                    .split(*area);
                if focus_name == focus {
                    frame.render_widget(
                        Paragraph::new(format!("{}: ", *label)).bg(Color::DarkGray),
                        fact_areas[0],
                    );
                } else {
                    frame.render_widget(Paragraph::new(format!("{}: ", *label)), fact_areas[0]);
                }
                let value = coffee
                    .as_ref()
                    .map(value_accessor)
                    .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
                frame.render_widget(Paragraph::new(value), fact_areas[1]);
            }
        });
    experience_fields
        .iter()
        .zip(experience_areas.iter())
        .for_each(|(field, area)| match field {
            DraftField::Header(label) => {
                frame.render_widget(Paragraph::new(format!("{}: ", *label)), *area)
            }
            DraftField::Field(label, focus_name, value_accessor) => {
                let experience_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(label.len() as u16 + 2),
                        Constraint::Min(0),
                    ])
                    .split(*area);
                if focus_name == focus {
                    frame.render_widget(
                        Paragraph::new(format!("{}: ", *label)).bg(Color::DarkGray),
                        experience_areas[0],
                    );
                } else {
                    frame.render_widget(
                        Paragraph::new(format!("{}: ", *label)),
                        experience_areas[0],
                    );
                }
                let value = coffee
                    .as_ref()
                    .map(value_accessor)
                    .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
                frame.render_widget(Paragraph::new(value), experience_areas[1]);
            }
            DraftField::Spacing => frame.render_widget(Paragraph::new("".to_string()), *area),
        });
    if let Some(message) = error {
        render_error(message, frame, inner_modal_area);
    }
}

fn build_fact_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
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

fn build_experience_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
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

fn update_draft_coffee(coffee: &mut Option<DraftCoffee>, focus: &AddFocus, keycode: KeyCode) {
    let coffee = coffee.get_or_insert_with(DraftCoffee::default);
    match keycode {
        KeyCode::Char(c) => match focus {
            AddFocus::Name => coffee.name.get_or_insert_with(String::new).push(c),
            AddFocus::Origin => coffee.origin.get_or_insert_with(String::new).push(c),
            AddFocus::Varieties => coffee.varieties.get_or_insert_with(String::new).push(c),
            AddFocus::Process => coffee.process.get_or_insert_with(String::new).push(c),
            AddFocus::Roaster => coffee.roaster.get_or_insert_with(String::new).push(c),
            // 5 (decaf) is handled in the event loop (Enter branch)
            AddFocus::DecaffeinationProcess => coffee
                .decaffeination_process
                .get_or_insert_with(String::new)
                .push(c),
            AddFocus::GrindSize => coffee
                .brew_settings
                .get_or_insert_with(DraftBrewSettings::default)
                .grind_size
                .get_or_insert_with(String::new)
                .push(c),
            AddFocus::AromaStrength => {
                let aroma = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aroma
                    .get_or_insert_with(Score::default);
                set_score(&mut aroma.strength, c);
            }
            AddFocus::AromaPersonal => {
                let aroma = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aroma
                    .get_or_insert_with(Score::default);
                set_score(&mut aroma.personal, c);
            }
            AddFocus::SweetnessStrength => {
                let sweetness = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .sweetness
                    .get_or_insert_with(Score::default);
                set_score(&mut sweetness.strength, c);
            }
            AddFocus::SweetnessPersonal => {
                let sweetness = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .sweetness
                    .get_or_insert_with(Score::default);
                set_score(&mut sweetness.personal, c);
            }
            AddFocus::AcidityStrength => {
                let acidity = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .acidity
                    .get_or_insert_with(Score::default);
                set_score(&mut acidity.strength, c);
            }
            AddFocus::AcidityPersonal => {
                let acidity = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .acidity
                    .get_or_insert_with(Score::default);
                set_score(&mut acidity.personal, c);
            }
            AddFocus::BodyStrength => {
                let body = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .body
                    .get_or_insert_with(Score::default);
                set_score(&mut body.strength, c);
            }
            AddFocus::BodyPersonal => {
                let body = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .body
                    .get_or_insert_with(Score::default);
                set_score(&mut body.personal, c);
            }
            AddFocus::AftertasteStrength => {
                let aftertaste = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aftertaste
                    .get_or_insert_with(Score::default);
                set_score(&mut aftertaste.strength, c);
            }
            AddFocus::AftertastePersonal => {
                let aftertaste = coffee
                    .rating
                    .get_or_insert_with(Rating::default)
                    .aftertaste
                    .get_or_insert_with(Score::default);
                set_score(&mut aftertaste.personal, c);
            }
            _ => (),
        },
        KeyCode::Backspace => match focus {
            AddFocus::Name => pop_optional_char(&mut coffee.name),
            AddFocus::Origin => pop_optional_char(&mut coffee.origin),
            AddFocus::Varieties => pop_optional_char(&mut coffee.varieties),
            AddFocus::Process => pop_optional_char(&mut coffee.process),
            AddFocus::Roaster => pop_optional_char(&mut coffee.roaster),
            // 5 (decaf) is handled in the event loop (Enter branch)
            AddFocus::DecaffeinationProcess => {
                pop_optional_char(&mut coffee.decaffeination_process)
            }
            AddFocus::GrindSize => {
                let dbs = coffee
                    .brew_settings
                    .get_or_insert_with(DraftBrewSettings::default);
                pop_optional_char(&mut dbs.grind_size);
                if dbs.grind_size.as_deref() == Some("") {
                    coffee.brew_settings = None;
                }
            }
            AddFocus::AromaStrength => {
                if let Some(aroma) = coffee.rating.as_mut().and_then(|r| r.aroma.as_mut()) {
                    aroma.strength = None;
                }
            }
            AddFocus::AromaPersonal => {
                if let Some(aroma) = coffee.rating.as_mut().and_then(|r| r.aroma.as_mut()) {
                    aroma.personal = None;
                }
            }
            AddFocus::SweetnessStrength => {
                if let Some(sweetness) = coffee.rating.as_mut().and_then(|r| r.sweetness.as_mut()) {
                    sweetness.strength = None;
                }
            }
            AddFocus::SweetnessPersonal => {
                if let Some(sweetness) = coffee.rating.as_mut().and_then(|r| r.sweetness.as_mut()) {
                    sweetness.personal = None;
                }
            }
            AddFocus::AcidityStrength => {
                if let Some(acidity) = coffee.rating.as_mut().and_then(|r| r.acidity.as_mut()) {
                    acidity.strength = None;
                }
            }
            AddFocus::AcidityPersonal => {
                if let Some(acidity) = coffee.rating.as_mut().and_then(|r| r.acidity.as_mut()) {
                    acidity.personal = None;
                }
            }
            AddFocus::BodyStrength => {
                if let Some(body) = coffee.rating.as_mut().and_then(|r| r.body.as_mut()) {
                    body.strength = None;
                }
            }
            AddFocus::BodyPersonal => {
                if let Some(body) = coffee.rating.as_mut().and_then(|r| r.body.as_mut()) {
                    body.personal = None;
                }
            }
            AddFocus::AftertasteStrength => {
                if let Some(aftertaste) = coffee.rating.as_mut().and_then(|r| r.aftertaste.as_mut())
                {
                    aftertaste.strength = None;
                }
            }
            AddFocus::AftertastePersonal => {
                if let Some(aftertaste) = coffee.rating.as_mut().and_then(|r| r.aftertaste.as_mut())
                {
                    aftertaste.personal = None;
                }
            }
            _ => (),
        },
        _ => (),
    }
}

fn pop_optional_char(field: &mut Option<String>) {
    if let Some(s) = field.as_mut() {
        s.pop();
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

fn convert_draft_to_coffee(draft: &DraftCoffee) -> Result<Coffee, Box<dyn std::error::Error>> {
    Ok(Coffee {
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
        ..Default::default()
    })
}

fn render_delete_coffee_modal(coffee: &Coffee, frame: &mut Frame, area: Rect) {
    let modal_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(50),
            Constraint::Fill(1),
        ])
        .split(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(10),
                    Constraint::Fill(1),
                ])
                .split(area)[1],
        )[1];
    let modal = Block::bordered().title("Delete coffee");
    let inner_modal_area = modal.inner(modal_area);
    frame.render_widget(Clear, modal_area);
    frame.render_widget(modal, modal_area);

    let mut coffee_fields = vec![
        format!("Name: {}", coffee.name),
        format!("Origin: {}", coffee.origin.as_deref().unwrap_or_default()),
        format!(
            "Varieties: {}",
            coffee.varieties.as_deref().unwrap_or_default().join(", ")
        ),
        format!("Process: {}", coffee.process.as_deref().unwrap_or_default()),
        format!("Roaster: {}", coffee.roaster.as_deref().unwrap_or_default()),
        format!(
            "Decaf: {}",
            if coffee.decaf.unwrap_or_default() {
                "☑"
            } else {
                "☐"
            }
        ),
    ];

    if let Some(decaf) = coffee.decaf
        && decaf
    {
        coffee_fields.push(format!(
            "Decaffeination process: {}",
            coffee.decaffeination_process.as_deref().unwrap_or_default()
        ))
    }

    let inner_modal_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(coffee_fields.len() as u16),
            Constraint::Min(0),
        ])
        .split(inner_modal_area);
    let field_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); coffee_fields.len()])
        .split(inner_modal_areas[0]);

    coffee_fields
        .iter()
        .zip(field_areas.iter())
        .for_each(|(value, area)| frame.render_widget(Paragraph::new(value.as_str()), *area));
}

fn render_error(error: &str, frame: &mut Frame, area: Rect) {
    let error_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area)[1];
    frame.render_widget(
        Paragraph::new(error)
            .fg(Color::Red)
            .alignment(HorizontalAlignment::Right),
        error_area,
    );
}
