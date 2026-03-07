mod draft;
mod fields;
mod render;

use std::path::Path;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{Event, KeyCode, read},
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};

use crate::app::{
    add_coffee, delete_coffee, filter_coffees, list_coffees, list_decaffeination_processes,
    list_origins, list_processes, list_roasters, list_varieties, update_coffee,
};
use crate::coffee::Coffee;
use crate::ui::draft::{
    DraftCoffee, convert_coffee_to_draft, convert_draft_to_coffee, pop_optional_char,
    update_draft_coffee,
};
use crate::ui::render::{
    render_app_name, render_coffees, render_delete_coffee_modal, render_error,
    render_input_coffee_modal, render_search_bar,
};

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
    query: Option<String>,
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

#[derive(Default, PartialEq)]
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
            InputFocus::AftertastePersonal => InputFocus::Name,
        }
    }

    fn previous(&self, coffee: &Option<DraftCoffee>) -> InputFocus {
        match self {
            InputFocus::Name => InputFocus::AftertastePersonal,
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

            let list = filter_coffees(&state.coffees, state.ui_state.query.as_deref());
            render_coffees(&mut state.ui_state.list_state, &list, frame, areas[1]);

            if state.ui_state.mode == Mode::Normal || state.ui_state.mode == Mode::Search {
                if let Some(message) = &state.ui_state.error {
                    render_error(message, frame, areas[1]);
                }
                if state.ui_state.mode == Mode::Search {
                    render_search_bar(
                        state.ui_state.query.as_deref().unwrap_or_default(),
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
                render_input_coffee_modal(
                    &mode_state.focus,
                    &mode_state.suggestions,
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
                    KeyCode::BackTab => {
                        state.ui_state.error = None;
                        if let Some(index) = state.ui_state.list_state.selected()
                            && index == 0
                        {
                            state.ui_state.list_state.select_last();
                        } else {
                            state.ui_state.list_state.select_previous();
                        }
                    }
                    KeyCode::Char('/') => {
                        state.ui_state.list_state.select(None);
                        state.ui_state.mode = Mode::Search;
                    }
                    KeyCode::Char('a') => {
                        state.ui_state.mode = Mode::Add;
                        state.ui_state.input_state = Some(InputModalState::default());
                        state.ui_state.error = None;
                    }
                    KeyCode::Char('e') | KeyCode::Char('u') => {
                        if state.ui_state.list_state.selected().is_none() {
                            state.ui_state.error = Some("Select a coffee to update it".to_string());
                        } else {
                            state.ui_state.input_state = Some(InputModalState {
                                coffee: Some(convert_coffee_to_draft(
                                    &state.coffees[state
                                        .ui_state
                                        .list_state
                                        .selected()
                                        .expect("dropped all beans")],
                                )),
                                ..InputModalState::default()
                            });
                            state.ui_state.error = None;
                            state.ui_state.mode = Mode::Update;
                        }
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
                Mode::Search => match key.code {
                    KeyCode::Esc => {
                        state.ui_state.query = None;
                        state.ui_state.mode = Mode::Normal;
                    }
                    KeyCode::Tab => {
                        if let Some(index) = state.ui_state.list_state.selected() {
                            let list =
                                filter_coffees(&state.coffees, state.ui_state.query.as_deref());
                            if index == list.len() - 1 {
                                state.ui_state.list_state.select(None);
                            } else {
                                state.ui_state.list_state.select_next();
                            }
                        } else {
                            state.ui_state.list_state.select_first();
                        }
                    }
                    KeyCode::BackTab => {
                        state.ui_state.error = None;
                        if let Some(index) = state.ui_state.list_state.selected() {
                            if index == 0 {
                                state.ui_state.list_state.select(None);
                            } else {
                                state.ui_state.list_state.select_previous();
                            }
                        } else {
                            state.ui_state.list_state.select_last();
                        }
                    }
                    KeyCode::Char(c) => {
                        if state.ui_state.list_state.selected().is_none() {
                            state.ui_state.query.get_or_insert_with(String::new).push(c);
                        } else {
                            match c {
                                'a' => {
                                    state.ui_state.mode = Mode::Add;
                                    state.ui_state.input_state = Some(InputModalState::default());
                                    state.ui_state.error = None;
                                }
                                'e' | 'u' => {
                                    let list = filter_coffees(
                                        &state.coffees,
                                        state.ui_state.query.as_deref(),
                                    );
                                    state.ui_state.input_state = Some(InputModalState {
                                        coffee: Some(convert_coffee_to_draft(
                                            list[state
                                                .ui_state
                                                .list_state
                                                .selected()
                                                .expect("dropped all beans")],
                                        )),
                                        ..InputModalState::default()
                                    });
                                    state.ui_state.error = None;
                                    state.ui_state.mode = Mode::Update;
                                }
                                'd' => state.ui_state.mode = Mode::Delete,

                                _ => (),
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        pop_optional_char(&mut state.ui_state.query);
                    }
                    _ => (),
                },
                Mode::Add | Mode::Update => {
                    let mode_state = state
                        .ui_state
                        .input_state
                        .as_mut()
                        .expect("mode state missing in mode");
                    match key.code {
                        KeyCode::Tab => {
                            mode_state.focus = mode_state.focus.next(&mode_state.coffee);
                            mode_state.suggestions = mode_state.focus.load_suggestions(&path);
                        }
                        KeyCode::BackTab => {
                            mode_state.focus = mode_state.focus.previous(&mode_state.coffee);
                            mode_state.suggestions = mode_state.focus.load_suggestions(&path);
                        }
                        KeyCode::Enter => {
                            if mode_state.focus == InputFocus::Decaf {
                                mode_state
                                    .coffee
                                    .get_or_insert_with(DraftCoffee::default)
                                    .toggle_decaf();
                            } else if let Some(draft) = &mode_state.coffee {
                                match convert_draft_to_coffee(draft) {
                                    Ok(coffee) => {
                                        if state.ui_state.mode == Mode::Add {
                                            add_coffee(coffee, &path)
                                                .expect("cannot remember coffee");
                                            state.ui_state.list_state = ListState::default();
                                        } else {
                                            update_coffee(coffee, &path)
                                                .expect("cannot improve coffee");
                                        }
                                        state.ui_state.input_state = None;
                                        state.coffees =
                                            list_coffees(&path).expect("could not list coffees");
                                        if state.ui_state.query.is_some() {
                                            state.ui_state.mode = Mode::Search;
                                        } else {
                                            state.ui_state.mode = Mode::Normal;
                                        }
                                    }
                                    Err(e) => state.ui_state.error = Some(e.to_string()),
                                }
                            }
                        }
                        KeyCode::Esc => {
                            state.ui_state.input_state = None;
                            state.ui_state.error = None;
                            state.ui_state.mode = Mode::Normal;
                        }
                        KeyCode::Backspace | KeyCode::Char(_) => {
                            if mode_state.focus == InputFocus::GrindSizeAdjustment {
                                if key.code == KeyCode::Char('+') {
                                    mode_state
                                        .coffee
                                        .as_mut()
                                        .expect("no coffee to adjust")
                                        .grind_coarser();
                                } else if key.code == KeyCode::Char('-') {
                                    mode_state
                                        .coffee
                                        .as_mut()
                                        .expect("no coffee to adjust")
                                        .grind_finer();
                                } else if key.code == KeyCode::Backspace {
                                    mode_state
                                        .coffee
                                        .as_mut()
                                        .expect("no coffee to adjust")
                                        .reset_grind_adjustment();
                                }
                            } else {
                                update_draft_coffee(
                                    &mut mode_state.coffee,
                                    &mode_state.focus,
                                    key.code,
                                );
                                if matches!(key.code, KeyCode::Char(_)) {
                                    state.ui_state.error = None;
                                }
                            }
                        }

                        _ => (),
                    }
                }
                Mode::Delete => match key.code {
                    KeyCode::Enter => {
                        let list = filter_coffees(&state.coffees, state.ui_state.query.as_deref());
                        delete_coffee(
                            list[state
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
                    KeyCode::Esc => {
                        if state.ui_state.query.is_some() {
                            state.ui_state.mode = Mode::Search;
                        } else {
                            state.ui_state.mode = Mode::Normal;
                        }
                    }
                    KeyCode::Char('q') => break Ok(()),
                    _ => (),
                },
            }
        }
    }
}
