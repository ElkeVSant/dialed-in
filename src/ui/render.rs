use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
};

use crate::coffee::Coffee;
use crate::ui::fields::{DraftField, build_experience_fields, build_fact_fields};
use crate::ui::{AddFocus, DraftCoffee};

const DIALED_IN: &str = r#"
8888888b. d8b        888             8888888888         
888  "Y88bY8P        888             888  888           
888    888           888             888  888           
888    888888 8888b. 888 .d88b.  .d88888  888  88888b.  
888    888888    "88b888d8P  Y8bd88" 888  888  888 "88b 
888    888888.d88888888888888888888  888  888  888  888 
888  .d88P888888  888888Y8b.    Y88b 888  888  888  888 
8888888P" 888"Y888888888 "Y8888  "Y888888888888888  888 "#;

pub fn render_app_name(frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new(DIALED_IN), area);
}

pub fn render_coffees(state: &mut ListState, coffees: &[Coffee], frame: &mut Frame, area: Rect) {
    let coffee_names: Vec<ListItem> = coffees
        .iter()
        .map(|c| ListItem::new(c.name.clone()))
        .collect();
    let coffee_list = List::new(coffee_names).highlight_style(Style::new().bg(Color::DarkGray));
    frame.render_stateful_widget(coffee_list, area, state);
}

pub fn render_add_coffee_modal(
    focus: &AddFocus,
    suggestions: &Option<Vec<String>>,
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
            if let DraftField::Field(label, focus_name, value_accessor, input_extractor) = field {
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
                // retrieve values from default DraftCoffee to ensure None values like the default
                // decaf checkbox are rendered as expected
                let value = coffee
                    .as_ref()
                    .map(value_accessor)
                    .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
                let mut suggestion_span = Span::raw("");
                if let Some(suggestions) = suggestions
                    && !value.is_empty()
                {
                    let match_input = match input_extractor {
                        Some(ie) => ie(&value),
                        None => value.clone(),
                    };
                    let suggestions: Vec<&String> = suggestions
                        .iter()
                        .filter(|s| s.starts_with(match_input.as_str()))
                        .collect();
                    if !suggestions.is_empty() {
                        let suggestion = &suggestions[0][match_input.len()..];
                        suggestion_span = Span::raw(suggestion).fg(Color::DarkGray);
                    }
                }
                let line = Line::from(vec![Span::raw(value).fg(Color::White), suggestion_span]);
                frame.render_widget(line, fact_areas[1]);
            }
        });
    experience_fields
        .iter()
        .zip(experience_areas.iter())
        .for_each(|(field, area)| match field {
            DraftField::Header(label) => {
                frame.render_widget(Paragraph::new(format!("{}: ", *label)), *area)
            }
            DraftField::Field(label, focus_name, value_accessor, _) => {
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
                // retrieve values from default DraftCoffee to ensure placeholders like the rating
                // circles are rendered as expected
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

pub fn render_delete_coffee_modal(coffee: &Coffee, frame: &mut Frame, area: Rect) {
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

pub fn render_error(error: &str, frame: &mut Frame, area: Rect) {
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
