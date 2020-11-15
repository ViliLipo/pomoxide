use crate::config::Config;
use crate::pomodoro;
use crate::util::event::{Event, Events};
use pomodoro::Session;
use std::collections::HashMap;
use termion::event::Key;

type ControlFunction = fn(&mut Session) -> ();

pub struct Controller {
    bindings: HashMap<char, ControlFunction>,
}

impl Controller {
    pub fn new(config: &Config) -> Controller {
        let mut bindings = HashMap::new();

        Controller { bindings: bindings }
    }

    pub fn handle_input(&self, events: &Events, session: &mut Session) {
        if let Ok(event) = events.next() {
            if let Event::Input(input) = event {
                handle_key(input, session);
            }
        }
    }

    pub fn handle_key(&self, input: Key, session: &mut Session) {
        match input {
            Key::Char(character) => {
                if let Some(control_action) = self.bindings.get(&character) {
                    control_action(session);
                }
            }
            _ => (),
        }
    }
}

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
