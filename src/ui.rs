use ratatui::crossterm::event::{read, Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{List, ListItem, Paragraph};
use ratatui::{DefaultTerminal, Frame};

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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let path = dirs::data_dir()
        .expect("could not find data directory")
        .join("Dialed In");
    let coffees = list_coffees(&path).expect("could not list coffees");

    loop {
        terminal.draw(|frame| {
            let areas = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(10), Constraint::Min(0)])
                .split(frame.area());

            render_app_name(frame, areas[0]);
            render_coffees(&coffees, frame, areas[1]);
        })?;

        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Char('q') {
                break Ok(());
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
