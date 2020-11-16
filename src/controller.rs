use crate::config::Config;
use crate::pomodoro;
use crate::util::event::{Event, Events};
use pomodoro::Session;
use std::collections::HashMap;
use termion::event::Key;

type ControlFunction = fn(&mut Session) -> ();

pub struct Control {
    name: String,
    binding: char,
    action: ControlFunction,
}

pub struct Controller {
    bindings: HashMap<char, Control>,
    description: String,
}

fn pause_function(session: &mut Session) {
    session.toggle_pause();
}

fn quit_function(session: &mut Session) {
    session.quit();
}

fn reset_function(session: &mut Session) {
    session.reset();
}

fn skip_function(session: &mut Session) {
    session.skip();
}

impl Controller {
    pub fn new(config: &Config) -> Controller {
        let bindings = Controller::get_bindings(Controller::get_controls(config));
        let description = Controller::build_description(&bindings);
        Controller {
            bindings: bindings,
            description: description,
        }
    }

    fn get_bindings(controls: Vec<Control>) -> HashMap<char, Control> {
        let mut bindings = HashMap::new();
        for control in controls {
            bindings.insert(control.binding, control);
        }
        return bindings;
    }

    fn get_controls(config: &Config) -> Vec<Control> {
        let mut controls = vec![];
        controls.push(Control {
            name: String::from("Pause"),
            binding: config.keybindings.pause,
            action: pause_function,
        });
        controls.push(Control {
            name: String::from("Quit"),
            binding: config.keybindings.quit,
            action: quit_function,
        });
        controls.push(Control {
            name: String::from("Reset"),
            binding: config.keybindings.reset,
            action: reset_function,
        });
        controls.push(Control {
            name: String::from("Skip"),
            binding: config.keybindings.skip,
            action: skip_function,
        });
        return controls;
    }

    pub fn handle_input(&self, events: &Events, session: &mut Session) {
        if let Ok(event) = events.next() {
            if let Event::Input(input) = event {
                self.handle_key(input, session);
            }
        }
    }

    pub fn handle_key(&self, input: Key, session: &mut Session) {
        match input {
            Key::Char(character) => {
                if let Some(control) = self.bindings.get(&character) {
                    (control.action)(session);
                }
            }
            _ => (),
        }
    }

    pub fn build_description(bindings: &HashMap<char, Control>) -> String{
        let mut string_parts = vec!();
        let mut value = String::new();
        value.push_str("Controls: ");
        for item in bindings.values() {
            string_parts.push(format!("{}:{}", item.name, item.binding));
        }
        string_parts.sort();
        for item in string_parts {
            value.push_str(item.as_str());
            value.push_str(", ");
        }
        value.truncate(value.len() -2);
        return value;
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }
}
