use ratatui::{
    crossterm::event::{read, Event, KeyCode},
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Stylize},
    widgets::{Block, List, ListItem, Paragraph},
    {DefaultTerminal, Frame},
};

use crate::app::list_coffees;
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

type FieldAccessor = Box<dyn Fn(&DraftCoffee) -> String>;
type Field = (&'static str, FieldAccessor);

#[derive(Default)]
struct State {
    coffees: Vec<Coffee>,
    ui_state: UiState,
}

#[derive(Default)]
struct UiState {
    mode: Mode,
    focus: usize,
    coffee: Option<DraftCoffee>,
}

#[derive(Default)]
enum Mode {
    #[default]
    Normal,
    Add,
}

#[derive(Default)]
pub struct DraftCoffee {
    pub name: Option<String>,
    pub origin: Option<String>,
    pub varieties: Option<String>,
    pub process: Option<String>,
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
            render_coffees(&state.coffees, frame, areas[1]);

            if matches!(state.ui_state.mode, Mode::Add) {
                render_add_coffee_modal(
                    state.ui_state.focus,
                    &state.ui_state.coffee,
                    frame,
                    areas[1],
                );
            }
        })?;

        if let Event::Key(key) = read()? {
            match state.ui_state.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('a') => state.ui_state.mode = Mode::Add,
                    _ => (),
                },
                Mode::Add => match key.code {
                    KeyCode::Tab => {
                        state.ui_state.focus =
                            (state.ui_state.focus + 1) % build_draft_fields().len()
                    }
                    KeyCode::Enter => break Ok(()),
                    KeyCode::Char(c) => {
                        update_draft_coffee(&mut state.ui_state.coffee, state.ui_state.focus, c);
                    }
                    _ => (),
                },
            }
        }
    }
}

fn render_app_name(frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new(DIALED_IN), area);
}

fn render_coffees(coffees: &[Coffee], frame: &mut Frame, area: Rect) {
    let coffee_names: Vec<ListItem> = coffees
        .iter()
        .map(|c| ListItem::new(c.name.clone()))
        .collect();
    let coffee_list = List::new(coffee_names);
    frame.render_widget(coffee_list, area);
}

fn render_add_coffee_modal(
    focus: usize,
    coffee: &Option<DraftCoffee>,
    frame: &mut Frame,
    area: Rect,
) {
    let modal_area = area.inner(Margin::new(2, 1));
    let modal = Block::bordered().title("New coffee");
    let field_area = modal.inner(modal_area);
    frame.render_widget(modal, modal_area);

    let field_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(field_area);
    build_draft_fields()
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
            let value = coffee.as_ref().map(value_accessor).unwrap_or_default();
            frame.render_widget(Paragraph::new(value), field_areas[1]);
        });
}

fn update_draft_coffee(coffee: &mut Option<DraftCoffee>, focus: usize, c: char) {
    let coffee = coffee.get_or_insert_with(DraftCoffee::default);
    match focus {
        0 => coffee.name.get_or_insert_with(String::new).push(c),
        1 => coffee.origin.get_or_insert_with(String::new).push(c),
        2 => coffee.varieties.get_or_insert_with(String::new).push(c),
        3 => coffee.process.get_or_insert_with(String::new).push(c),
        4 => coffee.roaster.get_or_insert_with(String::new).push(c),
        _ => (),
    };
}

fn build_draft_fields() -> Vec<Field> {
    vec![
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
    ]
}
