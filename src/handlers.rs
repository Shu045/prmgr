use crossterm::event::{KeyCode, KeyEvent};

use crate::action::Action;

pub fn key_to_action(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Up => Action::MoveUp,
        KeyCode::Down => Action::MoveDown,
        KeyCode::Char('k') => Action::KillProcess,
        KeyCode::Char('r') => Action::Refresh,
        KeyCode::Char('c') => Action::StartSearch,
        KeyCode::Esc => Action::StopSearch,
        KeyCode::Enter => Action::StopSearch,

        _ => Action::None,
    }
}
