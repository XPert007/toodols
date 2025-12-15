use crate::event::Event;
use color_eyre::eyre::{Ok, Result};
use color_eyre::owo_colors::{OwoColorize, Style};
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style as RStyle;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Padding};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event,
    widgets::{Paragraph, Widget},
};
#[derive(Debug, Default)]
struct AppState {
    items: Vec<TodoItem>,
    list_state: ListState,
    is_add_new: bool,
    input_value: String,
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;
        if let Event::Key(key) = event::read()? {
            if app_state.is_add_new {
                match handle_add_new(key, app_state) {
                    FormAction::None => {}
                    FormAction::Submit => {
                        app_state.items.push(TodoItem {
                            is_done: false,
                            description: app_state.input_value.clone(),
                        });
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }
                    FormAction::Escape => {
                        app_state.is_add_new = false;
                        app_state.input_value.clear();
                    }
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}
fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        event::KeyCode::Char(c) => {
            app_state.input_value.push(c);
        }
        event::KeyCode::Backspace => {
            app_state.input_value.pop();
        }
        event::KeyCode::Esc => {
            return FormAction::Escape;
        }
        event::KeyCode::Enter => {
            return FormAction::Submit;
        }
        _ => {}
    }
    FormAction::None
}
fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Char(char) => match char {
            'j' => {
                app_state.list_state.select_next();
            }
            'A' => {
                app_state.is_add_new = true;
            }
            'D' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            _ => {}
        },
        _ => {}
    }
    false
}
fn render(frame: &mut Frame, app_state: &mut AppState) {
    if app_state.is_add_new {
        Paragraph::new(app_state.input_value.as_str())
            .block(
                Block::bordered()
                    .fg(Color::Green)
                    .padding(Padding::uniform(1))
                    .border_type(BorderType::Rounded),
            )
            .render(frame.area(), frame.buffer_mut());
    } else {
        let [border_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(frame.area());
        let [inner_area] = Layout::vertical([Constraint::Fill(1)])
            .margin(1)
            .areas(border_area);
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .fg(Color::Yellow)
            .render(border_area, frame.buffer_mut());
        let list = List::new(
            app_state
                .items
                .iter()
                .map(|x| ListItem::from(x.description.as_str())),
        )
        .highlight_symbol(">")
        .highlight_style(RStyle::default().fg(Color::Green));
        frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
    }
}
