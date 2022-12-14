use crate::{actions, App};
use crossterm::event::KeyCode;

/// Input events handler
pub fn handle_events(app: &mut App, keycode: KeyCode) {
    match keycode {
        KeyCode::Esc => actions::exit_app(app),
        KeyCode::Up => actions::list_up(app),
        KeyCode::Down => actions::list_down(app),
        KeyCode::PageUp => actions::list_pgup(app),
        KeyCode::PageDown => actions::list_pgdn(app),
        KeyCode::Home => actions::list_home(app),
        KeyCode::End => actions::list_end(app),
        KeyCode::Char(c) => actions::filter_add_char(app, c),
        KeyCode::Backspace => actions::filter_del_char(app),
        KeyCode::Enter => actions::run_ssh_session(app),
        _ => {}
    }
}
