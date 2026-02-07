use crate::models::Screen;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Quit,
    Pause,
    Backspace,
    Char(char),
    None,
}

pub fn translate(event: KeyEvent, screen: Screen) -> Action {
    if event.kind != KeyEventKind::Press {
        return Action::None;
    }

    match screen {
        Screen::Training | Screen::TypingTest => translate_typing(event),
        Screen::Pause => translate_pause(event),
        _ => translate_menu(event),
    }
}

fn translate_menu(event: KeyEvent) -> Action {
    match event.code {
        KeyCode::Up => Action::Up,
        KeyCode::Down => Action::Down,
        KeyCode::Left => Action::Left,
        KeyCode::Right => Action::Right,
        KeyCode::Enter => Action::Select,
        KeyCode::Esc => Action::Back,
        KeyCode::Char('k') => Action::Up,
        KeyCode::Char('j') => Action::Down,
        KeyCode::Char('h') => Action::Left,
        KeyCode::Char('l') => Action::Right,
        KeyCode::Char('q') => Action::Quit,
        _ => Action::None,
    }
}

fn translate_pause(event: KeyEvent) -> Action {
    match event.code {
        KeyCode::Up => Action::Up,
        KeyCode::Down => Action::Down,
        KeyCode::Enter => Action::Select,
        KeyCode::Esc => Action::Back,
        KeyCode::Char('k') => Action::Up,
        KeyCode::Char('j') => Action::Down,
        KeyCode::Char('q') => Action::Quit,
        _ => Action::None,
    }
}

fn translate_typing(event: KeyEvent) -> Action {
    match event.code {
        KeyCode::Esc => Action::Pause,
        KeyCode::Backspace => Action::Backspace,
        KeyCode::Enter => Action::Char('\n'),
        KeyCode::Tab => Action::Char('\t'),
        KeyCode::Char(c) => {
            if event.modifiers.contains(KeyModifiers::CONTROL) {
                if c == 'q' {
                    return Action::Quit;
                }
                return Action::None;
            }
            Action::Char(c)
        }
        _ => Action::None,
    }
}
