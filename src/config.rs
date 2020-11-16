use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use dirs::home_dir;
use std::fs::File;
use std::io::prelude::*;

use toml;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Keybindings {
    pub pause: char,
    pub quit: char,
    pub reset: char,
    pub skip: char,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Config {
    pub work_duration: u64,
    pub break_duration: u64,
    pub keybindings: Keybindings,
    pub show_bindings: bool,
}

fn read_config_file() -> Result<String, ()> {
    if let Some(mut path) = home_dir() {
        path.push(".config/pomoxide-config.toml");
        if let Ok(mut file) = File::open(path) {
            let mut contents = String::new();
            if let Ok(_return_value) = file.read_to_string(&mut contents) {
                return Ok(contents);
            }
        };
    };
    Err(())
}

pub fn default_keybindings() -> Keybindings {
    Keybindings {
        pause: 's',
        quit: 'q',
        reset: 'd',
        skip: 'f',
    }
}

pub fn default_config() -> Config {
    Config {
        work_duration: 25,
        break_duration: 5,
        keybindings: default_keybindings(),
        show_bindings: true,
    }
}

pub fn read_config() -> Config {
    if let Ok(config_plain) = read_config_file() {
        if let Ok(config) = toml::from_str(config_plain.as_str()) {
            config
        } else {
            println!("Using default config, could not parse config.toml");
            default_config()
        }
    } else {
        println!("Using default config, could not read config.toml");
        default_config()
    }
}
