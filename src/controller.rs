use crate::util::event::{Event, Events};
use crate::pomodoro;
use termion::event::Key;
use pomodoro::Session;

pub fn handle_input(events: &Events, session: &mut Session) {
    if let Ok(event) = events.next() {
        if let Event::Input(input) = event {
            handle_key(input, session);
        }
    }
}

pub fn handle_key(input: Key, session: &mut Session) {
    match input {
        Key::Char(c) => match c {
            'q' => session.quit(),
            's' => session.toggle_pause(),
            'j' => session.reset(),
            'l' => session.skip(),
            _ => (),
        },
        _ => (),
    }
}
