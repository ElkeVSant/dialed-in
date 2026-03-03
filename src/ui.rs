use ratatui::{
    crossterm::event::{Event, KeyCode, read},
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
    {DefaultTerminal, Frame},
};

use crate::app::{add_coffee, delete_coffee, list_coffees};
use crate::coffee::Coffee;

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
type DraftField = (&'static str, DraftFieldAccessor);

#[derive(Default)]
struct State {
    coffees: Vec<Coffee>,
    ui_state: UiState,
}

#[derive(Default)]
struct UiState {
    mode: Mode,
    add_focus: usize,
    list_state: ListState,
    coffee: Option<DraftCoffee>,
    error: Option<String>,
}

#[derive(Default)]
enum Mode {
    #[default]
    Normal,
    Add,
    Delete,
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

            if matches!(state.ui_state.mode, Mode::Normal) {
                if let Some(message) = &state.ui_state.error {
                    render_error(message, frame, areas[1]);
                }
            } else if matches!(state.ui_state.mode, Mode::Add) {
                render_add_coffee_modal(
                    state.ui_state.add_focus,
                    &state.ui_state.coffee,
                    &state.ui_state.error,
                    frame,
                    areas[1],
                );
            } else if matches!(state.ui_state.mode, Mode::Delete) {
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
                        state.ui_state.add_focus = 0;
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
                        state.ui_state.add_focus = (state.ui_state.add_focus + 1)
                            % build_draft_fields(&state.ui_state.coffee).len()
                    }
                    KeyCode::Enter => {
                        if state.ui_state.add_focus == 5 {
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
                        update_draft_coffee(
                            &mut state.ui_state.coffee,
                            state.ui_state.add_focus,
                            key.code,
                        );
                        if matches!(key.code, KeyCode::Char(_)) {
                            state.ui_state.error = None;
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
    focus: usize,
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

    let draft_fields = build_draft_fields(coffee);

    let inner_modal_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(draft_fields.len() as u16)])
        .split(inner_modal_area);
    let field_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); draft_fields.len()])
        .split(inner_modal_areas[0]);
    draft_fields
        .iter()
        .enumerate()
        .zip(field_areas.iter())
        .for_each(|((index, (label, value_accessor)), area)| {
            let field_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(label.len() as u16 + 2),
                    Constraint::Min(0),
                ])
                .split(*area);
            if index == focus {
                frame.render_widget(
                    Paragraph::new(format!("{}: ", *label)).bg(Color::DarkGray),
                    field_areas[0],
                );
            } else {
                frame.render_widget(Paragraph::new(format!("{}: ", *label)), field_areas[0]);
            }
            let value = coffee
                .as_ref()
                .map(value_accessor)
                .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
            frame.render_widget(Paragraph::new(value), field_areas[1]);
        });
    if let Some(message) = error {
        render_error(message, frame, inner_modal_area);
    }
}

fn build_draft_fields(draft: &Option<DraftCoffee>) -> Vec<DraftField> {
    let mut fields: Vec<DraftField> = vec![
        ("Name", Box::new(|c| c.name.clone().unwrap_or_default())),
        ("Origin", Box::new(|c| c.origin.clone().unwrap_or_default())),
        (
            "Varieties",
            Box::new(|c| c.varieties.clone().unwrap_or_default()),
        ),
        (
            "Process",
            Box::new(|c| c.process.clone().unwrap_or_default()),
        ),
        (
            "Roaster",
            Box::new(|c| c.roaster.clone().unwrap_or_default()),
        ),
        (
            "Decaf",
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
        && matches!(draft.decaf, Some(true))
    {
        fields.push((
            "Decaffeination Process",
            Box::new(|c: &DraftCoffee| c.decaffeination_process.clone().unwrap_or_default()),
        ));
    }
    fields
}

fn update_draft_coffee(coffee: &mut Option<DraftCoffee>, focus: usize, keycode: KeyCode) {
    let coffee = coffee.get_or_insert_with(DraftCoffee::default);
    // indices are linked to build_draft_fields response
    match keycode {
        KeyCode::Char(c) => match focus {
            0 => coffee.name.get_or_insert_with(String::new).push(c),
            1 => coffee.origin.get_or_insert_with(String::new).push(c),
            2 => coffee.varieties.get_or_insert_with(String::new).push(c),
            3 => coffee.process.get_or_insert_with(String::new).push(c),
            4 => coffee.roaster.get_or_insert_with(String::new).push(c),
            // 5 (decaf) is handled in the event loop (Enter branch)
            6 => coffee
                .decaffeination_process
                .get_or_insert_with(String::new)
                .push(c),
            _ => (),
        },
        KeyCode::Backspace => match focus {
            0 => pop_optional_char(&mut coffee.name),
            1 => pop_optional_char(&mut coffee.origin),
            2 => pop_optional_char(&mut coffee.varieties),
            3 => pop_optional_char(&mut coffee.process),
            4 => pop_optional_char(&mut coffee.roaster),
            // 5 (decaf) is handled in the event loop (Enter branch)
            6 => pop_optional_char(&mut coffee.decaffeination_process),
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
