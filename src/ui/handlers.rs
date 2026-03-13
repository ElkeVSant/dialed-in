use std::path::Path;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{add_coffee, delete_coffee, list_coffees, update_coffee};
use crate::ui::draft::{
    DraftCoffee, accept_suggestion, convert_coffee_to_draft, convert_draft_to_coffee,
    get_match_input, update_draft_coffee,
};
use crate::ui::query::query_coffees;
use crate::ui::{InputFocus, InputModalState, ListState, Mode, State};

pub fn handle_normal_mode_events(state: &mut State, key: &KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => false,
        KeyCode::Tab | KeyCode::Down | KeyCode::Char('j') => {
            state.ui_state.error = None;
            if let Some(index) = state.ui_state.list_state.selected()
                && index == state.coffees.len() - 1
            {
                state.ui_state.list_state.select_first();
            } else {
                state.ui_state.list_state.select_next();
            }
            true
        }
        KeyCode::BackTab | KeyCode::Up | KeyCode::Char('k') => {
            state.ui_state.error = None;
            if let Some(index) = state.ui_state.list_state.selected()
                && index == 0
            {
                state.ui_state.list_state.select_last();
            } else {
                state.ui_state.list_state.select_previous();
            }
            true
        }
        KeyCode::Char('/') => {
            state.ui_state.list_state.select(None);
            state.ui_state.mode = Mode::Search;
            true
        }
        KeyCode::Char('g') => {
            state.ui_state.show_grind_size = !state.ui_state.show_grind_size;
            true
        }
        KeyCode::Char('a') => {
            state.ui_state.mode = Mode::Add;
            state.ui_state.input_state = Some(InputModalState::default());
            state.ui_state.error = None;
            true
        }
        KeyCode::Char('e') | KeyCode::Char('u') | KeyCode::Char(' ') => {
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
            true
        }
        KeyCode::Char('d') | KeyCode::Backspace => {
            if state.ui_state.list_state.selected().is_none() {
                state.ui_state.error = Some("Select a coffee to delete it".to_string());
            } else {
                state.ui_state.mode = Mode::Delete;
            }
            true
        }
        _ => true,
    }
}

pub fn handle_search_mode_events(state: &mut State, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            state.ui_state.search_state = None;
            state.ui_state.mode = Mode::Normal;
        }
        KeyCode::Tab | KeyCode::Down => {
            state.ui_state.error = None;
            select_next_search(state);
        }
        KeyCode::BackTab | KeyCode::Up => {
            state.ui_state.error = None;
            select_previous_search(state);
        }
        KeyCode::Char(c) => {
            if state.ui_state.list_state.selected().is_none() {
                state
                    .ui_state
                    .search_state
                    .get_or_insert_default()
                    .query
                    .push(c);
                state.ui_state.error = None;
            } else {
                match c {
                    'j' => select_next_search(state),
                    'k' => select_previous_search(state),
                    'g' => {
                        state.ui_state.show_grind_size = !state.ui_state.show_grind_size;
                    }
                    'a' => {
                        state.ui_state.mode = Mode::Add;
                        state.ui_state.input_state = Some(InputModalState::default());
                        state.ui_state.error = None;
                    }
                    'e' | 'u' | ' ' => {
                        match query_coffees(
                            &state.coffees,
                            state
                                .ui_state
                                .search_state
                                .as_ref()
                                .map(|s| s.query.as_str()),
                        ) {
                            Ok(list) => {
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
                            Err(e) => state.ui_state.error = Some(e.message),
                        }
                    }
                    'd' => state.ui_state.mode = Mode::Delete,

                    _ => (),
                }
            }
        }
        KeyCode::Backspace => {
            state.ui_state.error = None;
            if state.ui_state.list_state.selected().is_some() {
                state.ui_state.mode = Mode::Delete;
            } else if let Some(search_state) = &mut state.ui_state.search_state {
                search_state.query.pop();
            }
        }
        _ => (),
    }
}

fn select_next_search(state: &mut State) {
    if let Some(index) = state.ui_state.list_state.selected() {
        match query_coffees(
            &state.coffees,
            state
                .ui_state
                .search_state
                .as_ref()
                .map(|s| s.query.as_str()),
        ) {
            Ok(list) => {
                if index == list.len() - 1 {
                    state.ui_state.list_state.select(None);
                } else {
                    state.ui_state.list_state.select_next();
                }
            }
            Err(e) => state.ui_state.error = Some(e.message),
        }
    } else {
        state.ui_state.list_state.select_first();
    }
}

fn select_previous_search(state: &mut State) {
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

pub fn handle_input_modes_events(state: &mut State, key: &KeyEvent, path: &Path) {
    let mode_state = state
        .ui_state
        .input_state
        .as_mut()
        .expect("mode state missing in mode");
    match key.code {
        KeyCode::Tab | KeyCode::Down => {
            mode_state.focus = mode_state.focus.next(&mode_state.coffee);
            mode_state.suggestions = mode_state.focus.load_suggestions(path);
        }
        KeyCode::BackTab | KeyCode::Up => {
            mode_state.focus = mode_state.focus.previous(&mode_state.coffee);
            mode_state.suggestions = mode_state.focus.load_suggestions(path);
        }
        KeyCode::Left => mode_state.focus = InputFocus::Name,
        KeyCode::Right => mode_state.focus = InputFocus::AromaStrength,
        KeyCode::Enter => {
            let focus = mode_state.focus.clone();

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

            if mode_state.focus == InputFocus::Decaf {
                mode_state
                    .coffee
                    .get_or_insert_with(DraftCoffee::default)
                    .toggle_decaf();
            } else if suggestion.is_some()
                && let Some(coffee) = mode_state.coffee.as_mut()
            {
                accept_suggestion(coffee, &focus, suggestion.expect("suggestion disappeared"));
            } else if let Some(draft) = &mode_state.coffee {
                match convert_draft_to_coffee(draft) {
                    Ok(coffee) => {
                        if state.ui_state.mode == Mode::Add {
                            add_coffee(coffee, path).expect("cannot remember coffee");
                            state.ui_state.list_state = ListState::default();
                        } else {
                            update_coffee(coffee, path).expect("cannot improve coffee");
                        }
                        state.ui_state.input_state = None;
                        state.coffees = list_coffees(path).expect("could not list coffees");
                        restore_mode(state);
                    }
                    Err(e) => state.ui_state.error = Some(e.to_string()),
                }
            }
        }
        KeyCode::Esc => {
            state.ui_state.input_state = None;
            state.ui_state.error = None;
            restore_mode(state);
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
                update_draft_coffee(&mut mode_state.coffee, &mode_state.focus, key.code);
                if matches!(key.code, KeyCode::Char(_)) {
                    state.ui_state.error = None;
                }
            }
        }

        _ => (),
    }
}

pub fn handle_delete_mode_events(state: &mut State, key: &KeyEvent, path: &Path) -> bool {
    match key.code {
        KeyCode::Enter => match query_coffees(
            &state.coffees,
            state
                .ui_state
                .search_state
                .as_ref()
                .map(|s| s.query.as_str()),
        ) {
            Ok(list) => {
                delete_coffee(
                    list[state
                        .ui_state
                        .list_state
                        .selected()
                        .expect("dropped all beans")]
                    .id,
                    path,
                )
                .expect("cannot purge grinder");
                state.coffees = list_coffees(path).expect("could not list coffees");
                state.ui_state.list_state = ListState::default();
                restore_mode(state);
                true
            }
            Err(e) => {
                state.ui_state.error = Some(e.message);
                true
            }
        },
        KeyCode::Esc => {
            restore_mode(state);
            true
        }
        _ => true,
    }
}

fn restore_mode(state: &mut State) {
    if state.ui_state.search_state.is_some() {
        state.ui_state.mode = Mode::Search;
    } else {
        state.ui_state.mode = Mode::Normal;
    }
}
