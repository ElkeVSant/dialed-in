use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::ui::fields::{
    DraftField, build_coffee_fields, build_notes_field, build_rating_fields, calculate_score,
};
use crate::ui::style::{ACIDITY, AFTERTASTE, AROMA, BODY, SWEETNESS};
use crate::ui::{DraftCoffee, InputFocus};
use crate::{coffee::Coffee, ui::style::get_colour_for_total_score};

const DIALED_IN: &str = r#"
8888888b. d8b        888             8888888888         
888  "Y88bY8P        888             888  888           
888    888           888             888  888           
888    888888 8888b. 888 .d88b.  .d88888  888  88888b.  
888    888888    "88b888d8P  Y8bd88" 888  888  888 "88b 
888    888888.d88888888888888888888  888  888  888  888 
888  .d88P888888  888888Y8b.    Y88b 888  888  888  888 
8888888P" 888"Y888888888 "Y8888  "Y888888888888888  888 "#;

pub(super) fn render_app_name(frame: &mut Frame, area: Rect) {
    frame.render_widget(Paragraph::new(DIALED_IN), area);
}

pub(super) fn render_coffees(
    state: &mut ListState,
    show_grind_size: bool,
    coffees: &[&Coffee],
    frame: &mut Frame,
    area: Rect,
) {
    let list_areas = {
        if show_grind_size {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(10),
                    Constraint::Length(6),
                ])
                .split(area)
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(0), Constraint::Length(6)])
                .split(area)
        }
    };
    let mut names = Vec::new();
    let mut ratings = Vec::new();
    let mut grind_sizes = Vec::new();
    for coffee in coffees {
        names.push(coffee.name.clone());
        ratings.push({
            if let Some(rating) = coffee.rating
                && let Some(score) = calculate_score(&rating)
            {
                ListItem::new(format!("☕ {}", score)).style(
                    Style::new()
                        .bg(get_colour_for_total_score(score))
                        .fg(Color::Black),
                )
            } else {
                ListItem::new("".to_string())
            }
        });
        if show_grind_size {
            grind_sizes.push({
                if let Some(bs) = coffee.brew_settings {
                    bs.grind_size.to_string()
                } else {
                    "".to_string()
                }
            })
        }
    }
    let selected_style = Style::new().bg(Color::DarkGray);
    let coffee_list = List::new(names).highlight_style(selected_style);
    frame.render_stateful_widget(coffee_list, list_areas[0], state);
    let rating_list = List::new(ratings);
    frame.render_stateful_widget(rating_list, list_areas[list_areas.len() - 1], state);
    if show_grind_size {
        let grind_size_list = List::new(grind_sizes).highlight_style(selected_style);
        frame.render_stateful_widget(grind_size_list, list_areas[1], state);
    }
}

pub(super) fn render_input_coffee_modal(
    focus: &InputFocus,
    suggestion: &Option<String>,
    coffee: &Option<DraftCoffee>,
    error: &Option<String>,
    title: &str,
    frame: &mut Frame,
    area: Rect,
) {
    let modal_area = area.inner(Margin::new(2, 1));
    let modal = Block::bordered().title(title);
    let inner_modal_area = modal.inner(modal_area);
    frame.render_widget(Clear, modal_area);
    frame.render_widget(modal, modal_area);

    let coffee_fields = build_coffee_fields(coffee);
    let rating_fields = build_rating_fields(coffee);
    let notes_field = build_notes_field();

    let content_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(coffee_fields.len().max(rating_fields.len()) as u16),
            Constraint::Max(1),
            Constraint::Length(7),
        ])
        .split(inner_modal_area);
    let field_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
        .split(content_areas[0]);
    let coffee_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1);
            coffee_fields.len().max(rating_fields.len())
        ])
        .split(field_areas[0]);
    let rating_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1);
            coffee_fields.len().max(rating_fields.len())
        ])
        .split(field_areas[1]);

    coffee_fields
        .iter()
        .zip(coffee_areas.iter())
        .for_each(|(field, area)| {
            if let DraftField::Field(label, focus_name, value_accessor, input_extractor) = field {
                let coffee_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(label.len() as u16 + 2),
                        Constraint::Min(0),
                    ])
                    .split(*area);
                if focus_name == focus {
                    frame.render_widget(
                        Paragraph::new(format!("{}: ", *label)).bg(Color::DarkGray),
                        coffee_areas[0],
                    );
                } else {
                    frame.render_widget(Paragraph::new(format!("{}: ", *label)), coffee_areas[0]);
                }
                // retrieve values from default DraftCoffee to ensure None values like the default
                // decaf checkbox are rendered as expected
                let value = coffee
                    .as_ref()
                    .map(value_accessor)
                    .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
                let suggestion_span = {
                    if let Some(s) = suggestion
                        && focus_name == focus
                    {
                        let match_input = match input_extractor {
                            Some(ie) => ie(&value),
                            None => value.clone(),
                        };
                        let display_suggestion = &s[match_input.len()..];
                        Span::raw(display_suggestion).fg(Color::DarkGray)
                    } else {
                        Span::raw("")
                    }
                };
                let display_value = if focus_name == focus && focus != &InputFocus::Decaf {
                    value + "|"
                } else {
                    value
                };
                let line = Line::from(vec![
                    Span::raw(display_value).fg(Color::White),
                    suggestion_span,
                ]);
                frame.render_widget(line, coffee_areas[1]);
            }
        });
    rating_fields
        .iter()
        .zip(rating_areas.iter())
        .for_each(|(field, area)| match field {
            DraftField::Header(label) => {
                frame.render_widget(Paragraph::new(format!("{}: ", *label)), *area)
            }
            DraftField::ScoreField(label, focus_name, value_accessor) => {
                let rating_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(label.len() as u16 + 2),
                        Constraint::Min(0),
                    ])
                    .split(*area);
                if focus_name == focus {
                    frame.render_widget(
                        Paragraph::new(format!("{}: ", *label)).bg(Color::DarkGray),
                        rating_areas[0],
                    );
                } else {
                    frame.render_widget(Paragraph::new(format!("{}: ", *label)), rating_areas[0]);
                }
                // retrieve values from default DraftCoffee to ensure placeholders like the rating
                // circles are rendered as expected
                let value = coffee
                    .as_ref()
                    .map(value_accessor)
                    .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
                frame.render_widget(
                    Paragraph::new(format_score(value)).fg(determine_colour(focus_name, value)),
                    rating_areas[1],
                );
            }
            DraftField::Spacing => frame.render_widget(Paragraph::new("".to_string()), *area),
            DraftField::Summary(label, value_accessor) => {
                let value = coffee
                    .as_ref()
                    .map(value_accessor)
                    .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
                let summary_areas = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(label.len() as u16 + 2),
                        Constraint::Length(if value < 10 { 1 } else { 2 } + 4),
                        Constraint::Min(0),
                    ])
                    .split(*area);
                frame.render_widget(Paragraph::new(format!("{}: ", *label)), summary_areas[0]);
                let score_colour = get_colour_for_total_score(value);
                frame.render_widget(
                    Paragraph::new(format!("☕ {} ", value))
                        .bg(score_colour)
                        .fg(Color::Black),
                    summary_areas[1],
                );
            }
            _ => unreachable!(),
        });

    if let DraftField::Field(label, focus_name, value_accessor, _) = &notes_field {
        let notes_area = content_areas[2].inner(Margin::new(2, 1));
        let notes_frame = {
            if focus == focus_name {
                Block::bordered()
                    .title(format!("{}: ", label))
                    .title_style(Style::default().bg(Color::DarkGray))
            } else {
                Block::bordered().title(format!("{}: ", label))
            }
        };
        let notes_inner_area = notes_frame.inner(notes_area);
        frame.render_widget(notes_frame, notes_area);
        let value = coffee
            .as_ref()
            .map(value_accessor)
            .unwrap_or_else(|| value_accessor(&DraftCoffee::default()));
        let display_value = if focus_name == focus {
            value + "|"
        } else {
            value
        };
        frame.render_widget(
            Paragraph::new(display_value).wrap(Wrap { trim: true }),
            notes_inner_area,
        );
    }

    if let Some(message) = error {
        render_error(message, frame, inner_modal_area);
    }
}

fn determine_colour(focus: &InputFocus, score: u8) -> Color {
    match focus {
        InputFocus::AromaStrength | InputFocus::AromaPersonal => AROMA.for_score(score),
        InputFocus::SweetnessStrength | InputFocus::SweetnessPersonal => SWEETNESS.for_score(score),
        InputFocus::AcidityStrength | InputFocus::AcidityPersonal => ACIDITY.for_score(score),
        InputFocus::BodyStrength | InputFocus::BodyPersonal => BODY.for_score(score),
        InputFocus::AftertasteStrength | InputFocus::AftertastePersonal => {
            AFTERTASTE.for_score(score)
        }
        _ => Color::default(),
    }
}

fn format_score(score: u8) -> String {
    format!(
        "{}{}",
        "● ".repeat(score as usize),
        "○ ".repeat((5 - score) as usize)
    )
}

pub(super) fn render_delete_coffee_modal(coffee: &Coffee, frame: &mut Frame, area: Rect) {
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

pub(super) fn render_error(error: &str, frame: &mut Frame, area: Rect) {
    let error_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(error.len() as u16)])
        .split(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(area)[1],
        )[1];
    frame.render_widget(
        Paragraph::new(error)
            .fg(Color::Red)
            .alignment(HorizontalAlignment::Right),
        error_area,
    );
}

pub(super) fn render_search_bar(
    selection: &Option<usize>,
    query: &str,
    suggestion: &Option<String>,
    frame: &mut Frame,
    area: Rect,
) {
    let bar = Block::bordered().title("Search");
    let inner_search_area = bar.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(bar, area);
    if selection.is_none() {
        let suggestion_span = {
            if let Some(s) = suggestion {
                let display_suggestion = &s[query.len()..];
                Span::raw(display_suggestion).fg(Color::DarkGray)
            } else {
                Span::raw("")
            }
        };
        let line = Line::from(vec![
            Span::raw(format!("{}|", query)).fg(Color::White),
            suggestion_span,
        ]);
        frame.render_widget(line, inner_search_area);
    } else {
        frame.render_widget(Paragraph::new(query), inner_search_area);
    }
}

pub(super) fn render_help_panel(frame: &mut Frame, area: Rect) {
    let help_panel_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(12)])
        .split(
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(0), Constraint::Length(64)])
                .split(area)[1],
        )[1]
    .inner(Margin::new(2, 1));
    let panel = Block::new().title("Options: ?");
    let inner_panel_area = panel.inner(help_panel_area);
    let inner_panel = Block::new().bg(Color::Gray);

    frame.render_widget(Clear, help_panel_area);
    frame.render_widget(panel, help_panel_area);
    frame.render_widget(inner_panel, inner_panel_area);

    let options = vec![
        ("a", "add coffee", "/", "search"),
        ("e | u | _", "update coffee", "", ""),
        ("d", "delete coffee", "ar", "aroma"),
        ("", "", "sw", "sweetness"),
        ("Tab | ↓", "select next", "ac", "acidity"),
        ("Shift + Tab | ↑", "select previous", "bo", "body"),
        ("←", "select left column", "af", "aftertaste"),
        ("→", "select right column", ".s", "strength"),
        ("", "", ".p", "personal"),
        ("Esc | q", "close modal/quit app", ":a+", "at least a"),
        ("?", "open/close help panel", ":a-", "at most a"),
    ];
    let panel_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1); options.len()])
        .split(inner_panel_area);

    options
        .iter()
        .zip(panel_rows.iter())
        .for_each(|((key, option, filter, meaning), row_area)| {
            let row_areas = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Length(16),
                    Constraint::Fill(1),
                    Constraint::Length(22),
                    Constraint::Fill(4),
                    Constraint::Length(4),
                    Constraint::Fill(1),
                    Constraint::Length(12),
                ])
                .split(*row_area);
            frame.render_widget(Paragraph::new(*key).fg(Color::Black), row_areas[0]);
            frame.render_widget(Paragraph::new(*option).fg(Color::Black), row_areas[2]);
            frame.render_widget(Paragraph::new(*filter).fg(Color::Black), row_areas[4]);
            frame.render_widget(Paragraph::new(*meaning).fg(Color::Black), row_areas[6]);
        });
}
