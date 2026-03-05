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
    add_coffee, delete_coffee, list_coffees, list_decaffeination_processes, list_origins,
    list_processes, list_roasters, list_varieties,
};
use crate::coffee::Coffee;
use crate::ui::draft::{DraftCoffee, convert_draft_to_coffee, update_draft_coffee};
use crate::ui::render::{
    render_add_coffee_modal, render_app_name, render_coffees, render_delete_coffee_modal,
    render_error,
};

#[derive(Default)]
struct State {
    coffees: Vec<Coffee>,
    ui_state: UiState,
}

#[derive(Default)]
struct UiState {
    mode: Mode,
    add_state: Option<AddState>,
    list_state: ListState,
    error: Option<String>,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Normal,
    Add,
    Delete,
}

#[derive(Default)]
struct AddState {
    focus: AddFocus,
    coffee: Option<DraftCoffee>,
    suggestions: Option<Vec<String>>,
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

    fn load_suggestions(&self, path: &Path) -> Option<Vec<String>> {
        match self {
            AddFocus::Process => list_processes(path).ok(),
            AddFocus::Origin => list_origins(path).ok(),
            AddFocus::Varieties => list_varieties(path).ok(),
            AddFocus::Roaster => list_roasters(path).ok(),
            AddFocus::DecaffeinationProcess => list_decaffeination_processes(path).ok(),
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
                let add_state = state
                    .ui_state
                    .add_state
                    .get_or_insert_with(AddState::default);
                render_add_coffee_modal(
                    &add_state.focus,
                    &add_state.suggestions,
                    &add_state.coffee,
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
                        state.ui_state.add_state = Some(AddState::default());
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
                Mode::Add => {
                    let add_state = state
                        .ui_state
                        .add_state
                        .as_mut()
                        .expect("add state missing in Add mode");
                    match key.code {
                        KeyCode::Tab => {
                            add_state.focus = add_state.focus.next(&add_state.coffee);
                            add_state.suggestions = add_state.focus.load_suggestions(&path);
                        }
                        KeyCode::Enter => {
                            if add_state.focus == AddFocus::Decaf {
                                add_state
                                    .coffee
                                    .get_or_insert_with(DraftCoffee::default)
                                    .toggle_decaf();
                            } else if let Some(draft) = &add_state.coffee {
                                match convert_draft_to_coffee(draft) {
                                    Ok(coffee) => {
                                        add_coffee(coffee, &path).expect("cannot remember coffee");
                                        state.coffees =
                                            list_coffees(&path).expect("could not list coffees");
                                        state.ui_state.add_state = None;
                                        state.ui_state.list_state = ListState::default();
                                        state.ui_state.mode = Mode::Normal;
                                    }
                                    Err(e) => state.ui_state.error = Some(e.to_string()),
                                }
                            }
                        }
                        KeyCode::Esc => {
                            state.ui_state.add_state = None;
                            state.ui_state.error = None;
                            state.ui_state.mode = Mode::Normal;
                        }
                        KeyCode::Backspace | KeyCode::Char(_) => {
                            if add_state.focus == AddFocus::GrindSizeAdjustment {
                                if key.code == KeyCode::Char('+') {
                                    add_state
                                        .coffee
                                        .as_mut()
                                        .expect("no coffee to adjust")
                                        .grind_coarser();
                                } else if key.code == KeyCode::Char('-') {
                                    add_state
                                        .coffee
                                        .as_mut()
                                        .expect("no coffee to adjust")
                                        .grind_finer();
                                } else if key.code == KeyCode::Backspace {
                                    add_state
                                        .coffee
                                        .as_mut()
                                        .expect("no coffee to adjust")
                                        .reset_grind_adjustment();
                                }
                            } else {
                                update_draft_coffee(
                                    &mut add_state.coffee,
                                    &add_state.focus,
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
