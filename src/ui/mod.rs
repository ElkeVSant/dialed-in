mod draft;
mod fields;
mod handlers;
mod query;
mod render;
mod style;

use std::path::Path;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event, read},
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

use crate::app::{
    list_coffees, list_decaffeination_processes, list_origins, list_processes, list_roasters,
    list_varieties,
};
use crate::ui::handlers::{handle_delete_mode_events, handle_normal_mode_events};
use crate::ui::query::query_coffees;
use crate::ui::render::{
    render_app_name, render_coffees, render_delete_coffee_modal, render_error,
    render_input_coffee_modal, render_search_bar,
};
use crate::ui::{
    draft::{DraftCoffee, get_match_input},
    handlers::handle_input_modes_events,
};
use crate::{coffee::Coffee, ui::handlers::handle_search_mode_events};

#[derive(Default)]
struct State {
    coffees: Vec<Coffee>,
    ui_state: UiState,
}

#[derive(Default)]
struct UiState {
    mode: Mode,
    input_state: Option<InputModalState>,
    list_state: ListState,
    search_state: Option<SearchState>,
    show_grind_size: bool,
    error: Option<String>,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Normal,
    Add,
    Update,
    Delete,
    Search,
}

#[derive(Default)]
struct InputModalState {
    focus: InputFocus,
    coffee: Option<DraftCoffee>,
    suggestions: Option<Vec<String>>,
}

#[derive(Clone, Default, PartialEq)]
enum InputFocus {
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
    Notes,
}

#[derive(Default)]
struct SearchState {
    query: String,
    suggestions: Option<Vec<String>>,
}

impl InputFocus {
    fn next(&self, coffee: &Option<DraftCoffee>) -> InputFocus {
        match self {
            InputFocus::Name => InputFocus::Origin,
            InputFocus::Origin => InputFocus::Varieties,
            InputFocus::Varieties => InputFocus::Process,
            InputFocus::Process => InputFocus::Roaster,
            InputFocus::Roaster => InputFocus::Decaf,
            InputFocus::Decaf => {
                if let Some(coffee) = coffee
                    && coffee.decaf.unwrap_or_default()
                {
                    InputFocus::DecaffeinationProcess
                } else {
                    InputFocus::DecaffeinationProcess.next(coffee)
                }
            }
            InputFocus::DecaffeinationProcess => InputFocus::GrindSize,
            InputFocus::GrindSize => {
                if let Some(coffee) = coffee
                    && coffee.brew_settings.is_some()
                {
                    InputFocus::GrindSizeAdjustment
                } else {
                    InputFocus::GrindSizeAdjustment.next(coffee)
                }
            }
            InputFocus::GrindSizeAdjustment => InputFocus::AromaStrength,
            InputFocus::AromaStrength => InputFocus::AromaPersonal,
            InputFocus::AromaPersonal => InputFocus::SweetnessStrength,
            InputFocus::SweetnessStrength => InputFocus::SweetnessPersonal,
            InputFocus::SweetnessPersonal => InputFocus::AcidityStrength,
            InputFocus::AcidityStrength => InputFocus::AcidityPersonal,
            InputFocus::AcidityPersonal => InputFocus::BodyStrength,
            InputFocus::BodyStrength => InputFocus::BodyPersonal,
            InputFocus::BodyPersonal => InputFocus::AftertasteStrength,
            InputFocus::AftertasteStrength => InputFocus::AftertastePersonal,
            InputFocus::AftertastePersonal => InputFocus::Notes,
            InputFocus::Notes => InputFocus::Name,
        }
    }

    fn previous(&self, coffee: &Option<DraftCoffee>) -> InputFocus {
        match self {
            InputFocus::Name => InputFocus::Notes,
            InputFocus::Notes => InputFocus::AftertastePersonal,
            InputFocus::Origin => InputFocus::Name,
            InputFocus::Varieties => InputFocus::Origin,
            InputFocus::Process => InputFocus::Varieties,
            InputFocus::Roaster => InputFocus::Process,
            InputFocus::Decaf => InputFocus::Roaster,
            InputFocus::DecaffeinationProcess => InputFocus::Decaf,
            InputFocus::GrindSize => {
                if let Some(coffee) = coffee
                    && coffee.decaf.unwrap_or_default()
                {
                    InputFocus::DecaffeinationProcess
                } else {
                    InputFocus::DecaffeinationProcess.previous(coffee)
                }
            }
            InputFocus::GrindSizeAdjustment => InputFocus::GrindSize,
            InputFocus::AromaStrength => {
                if let Some(coffee) = coffee
                    && coffee.brew_settings.is_some()
                {
                    InputFocus::GrindSizeAdjustment
                } else {
                    InputFocus::GrindSizeAdjustment.previous(coffee)
                }
            }
            InputFocus::AromaPersonal => InputFocus::AromaStrength,
            InputFocus::SweetnessStrength => InputFocus::AromaPersonal,
            InputFocus::SweetnessPersonal => InputFocus::SweetnessStrength,
            InputFocus::AcidityStrength => InputFocus::SweetnessPersonal,
            InputFocus::AcidityPersonal => InputFocus::AcidityStrength,
            InputFocus::BodyStrength => InputFocus::AcidityPersonal,
            InputFocus::BodyPersonal => InputFocus::BodyStrength,
            InputFocus::AftertasteStrength => InputFocus::BodyPersonal,
            InputFocus::AftertastePersonal => InputFocus::AftertasteStrength,
        }
    }

    fn load_suggestions(&self, path: &Path) -> Option<Vec<String>> {
        match self {
            InputFocus::Process => list_processes(path).ok(),
            InputFocus::Origin => list_origins(path).ok(),
            InputFocus::Varieties => list_varieties(path).ok(),
            InputFocus::Roaster => list_roasters(path).ok(),
            InputFocus::DecaffeinationProcess => list_decaffeination_processes(path).ok(),
            _ => None,
        }
    }
}

pub(super) fn run() -> Result<(), Box<dyn std::error::Error>> {
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
            let areas = if state.ui_state.mode == Mode::Search {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(10),
                        Constraint::Length(3),
                        Constraint::Min(0),
                    ])
                    .split(frame.area())
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(10), Constraint::Min(0)])
                    .split(frame.area())
            };

            render_app_name(frame, areas[0]);

            let list = match query_coffees(
                &state.coffees,
                state
                    .ui_state
                    .search_state
                    .as_ref()
                    .map(|s| s.query.as_str()),
            ) {
                Ok(list) => {
                    render_coffees(
                        &mut state.ui_state.list_state,
                        state.ui_state.show_grind_size,
                        &list,
                        frame,
                        areas[areas.len() - 1],
                    );
                    list
                }
                Err(e) => {
                    state.ui_state.error = Some(e.message);
                    Vec::new()
                }
            };

            if state.ui_state.mode == Mode::Normal || state.ui_state.mode == Mode::Search {
                if let Some(message) = &state.ui_state.error {
                    render_error(message, frame, areas[areas.len() - 1]);
                }
                if state.ui_state.mode == Mode::Search {
                    let suggestion = {
                        if let Some(search_state) = &state.ui_state.search_state
                            && let Some(suggestions) = &search_state.suggestions
                            && !search_state.query.is_empty()
                        {
                            suggestions
                                .iter()
                                .find(|s| s.starts_with(search_state.query.as_str()))
                                .map(|s| s.to_string())
                        } else {
                            None
                        }
                    };
                    render_search_bar(
                        &state.ui_state.list_state.selected(),
                        state
                            .ui_state
                            .search_state
                            .as_ref()
                            .map(|s| s.query.as_str())
                            .unwrap_or_default(),
                        &suggestion,
                        frame,
                        areas[1],
                    );
                }
            } else if state.ui_state.mode == Mode::Add || state.ui_state.mode == Mode::Update {
                let title = if state.ui_state.mode == Mode::Add {
                    "New coffee"
                } else {
                    "Update coffee"
                };
                let mode_state = state
                    .ui_state
                    .input_state
                    .get_or_insert_with(InputModalState::default);

                let mut suggestion: Option<String> = None;
                if let Some(suggestions) = mode_state.suggestions.as_ref()
                    && let Some(coffee) = &mode_state.coffee
                {
                    let potential_match = get_match_input(&mode_state.focus, coffee);
                    if let Some(pm) = potential_match {
                        suggestion = suggestions
                            .iter()
                            .find(|s| s.starts_with(&pm))
                            .map(|s| s.to_string());
                    }
                }
                render_input_coffee_modal(
                    &mode_state.focus,
                    &suggestion,
                    &mode_state.coffee,
                    &state.ui_state.error,
                    title,
                    frame,
                    areas[1],
                );
            } else if state.ui_state.mode == Mode::Delete {
                render_delete_coffee_modal(
                    list[state.ui_state.list_state.selected().expect("no selection")],
                    frame,
                    areas[1],
                );
            }
        })?;

        if let Event::Key(key) = read()? {
            match state.ui_state.mode {
                Mode::Normal => {
                    if !handle_normal_mode_events(&mut state, &key) {
                        break Ok(());
                    }
                }
                Mode::Search => handle_search_mode_events(&mut state, &key, &path),
                Mode::Add | Mode::Update => handle_input_modes_events(&mut state, &key, &path),
                Mode::Delete => {
                    if !handle_delete_mode_events(&mut state, &key, &path) {
                        break Ok(());
                    }
                }
            }
        }
    }
}
