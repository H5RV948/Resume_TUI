use crossterm::{event::{self, Event, KeyCode, KeyEvent}};
use std::io::Result;

#[allow(dead_code)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
    Quit,
    Resize(u16,u16),
}

pub fn handle_action() -> Result<Option<Action>> {
    let ev = event::read()?;
    
    let action = match ev {
        Event::Key(KeyEvent {code, ..}) => match code {
            KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => Some(Action::Up),
            KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => Some(Action::Down),
            KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => Some(Action::Right),
            KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => Some(Action::Left),
            KeyCode::Char('q') | KeyCode::Esc => Some(Action::Quit),
            
            _ => None,
        }
        Event::Resize(w, h) => Some(Action::Resize(w, h)),

        _ => None,
    };

    Ok(action)
}