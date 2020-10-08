use crate::pomodoro;
use std::convert::TryInto;
use pomodoro::{Phase, Session};

pub struct Presentation {
    pub time_left: String,
    pub paused: String,
    pub tomatoes_done: String,
    pub phase: String,
    pub title: String,
}

impl Presentation {
    pub fn new(session: &Session) -> Presentation {
        Presentation {
            time_left: time_string_from_seconds(session.get_time_remaining()),
            paused: paused_string(session.is_paused()),
            tomatoes_done: tomatoes_done(session.get_completed_tomatoes()),
            phase: phase_string(session.get_current_phase()),
            title: String::from("Pomodoro"),
        }
    }
}


fn tomatoes_done(count: u64) -> String {
    format!("🍅 X {}", count)
}

fn time_in_minutes_and_seconds(seconds: u128) -> (u64, u64) {
    let minutes: u64 = match (seconds / 60).try_into() {
        Ok(val) => val,
        Err(_msg) => 0,
    };
    let seconds: u64 = match (seconds % 60).try_into() {
        Ok(val) => val,
        Err(_msg) => 0,
    };
    return (minutes, seconds);
}

fn time_to_string(time: (u64, u64)) -> String {
    String::from(format!("Time left: {:02}:{:02}", time.0, time.1))
}

fn time_string_from_seconds(seconds: u128) -> String {
    time_to_string(time_in_minutes_and_seconds(seconds))
}

fn paused_string(is_paused: bool) -> String {
    if is_paused {
        String::from("Paused")
    } else {
        String::from("")
    }
}

fn phase_string(phase: Phase) -> String {
    match phase {
        Phase::Break => String::from("You are on a break."),
        Phase::Tomato => String::from("You are working.")
    }
}


