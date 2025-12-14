use crate::event::Event;
use color_eyre::eyre::{Ok, Result};
use color_eyre::owo_colors::{OwoColorize, Style};
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style as RStyle;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Block, List, ListItem, ListState};
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
}

#[derive(Debug, Default)]
struct TodoItem {
    is_done: bool,
    description: String,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    state.is_add_new = false;
    color_eyre::install()?;
    state.items.push(TodoItem {
        is_done: true,
        description: "Hello".to_string(),
    });
    state.items.push(TodoItem {
        is_done: false,
        description: "jediowajer".to_string(),
    });
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc => {
                    break;
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
        }
    }
    Ok(())
}

fn handle_key(key: KeyEvent, app_state: &mut AppState) {}

fn render(frame: &mut Frame, app_state: &mut AppState) {
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
            .map(|x| ListItem::from(x.description.clone())),
    )
    .highlight_symbol(">")
    .highlight_style(RStyle::default().fg(Color::Green));
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);
}
