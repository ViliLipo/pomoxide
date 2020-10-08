use notify_rust::Notification;
use std::time::SystemTime;

#[derive(Clone, Copy)]
pub enum Phase {
    Tomato,
    Break,
}

#[derive(Copy)]
struct TimerState {
    time_elapsed_in_millis: u128,
    paused: bool,
    previous_checking_time: SystemTime,
    duration: u128,
}

impl Clone for TimerState {
    fn clone(&self) -> TimerState {
        *self
    }
}

impl TimerState {
    fn new(timer_type: Phase) -> TimerState {
        let break_duration: u128 = 5 * 60 * 1000;
        let tomato_duration: u128 = 25 * 60 * 1000;
        let duration = match timer_type {
            Phase::Break => break_duration,
            Phase::Tomato => tomato_duration,
        };
        TimerState {
            duration: duration,
            paused: true,
            time_elapsed_in_millis: 0,
            previous_checking_time: SystemTime::now(),
        }
    }

    fn toggle_pause(&self) -> TimerState {
        if self.paused {
            self.resume()
        } else {
            self.pause()
        }
    }

    fn pause(&self) -> TimerState {
        TimerState {
            paused: true,
            time_elapsed_in_millis: self.time_elapsed_in_millis,
            previous_checking_time: self.previous_checking_time,
            duration: self.duration,
        }
    }

    fn resume(&self) -> TimerState {
        TimerState {
            time_elapsed_in_millis: self.time_elapsed_in_millis,
            paused: false,
            previous_checking_time: self.previous_checking_time,
            duration: self.duration,
        }
    }

    fn update(&self) -> TimerState {
        let time_now = SystemTime::now();
        if self.paused {
            TimerState {
                time_elapsed_in_millis: self.time_elapsed_in_millis,
                paused: true,
                previous_checking_time: time_now,
                duration: self.duration,
            }
        } else {
            let updated_time = match time_now.duration_since(self.previous_checking_time) {
                Ok(time) => self.time_elapsed_in_millis + time.as_millis(),
                Err(_time_error) => self.time_elapsed_in_millis,
            };
            TimerState {
                time_elapsed_in_millis: updated_time,
                paused: false,
                previous_checking_time: time_now,
                duration: self.duration,
            }
        }
    }

    fn get_time_remaining(&self) -> u128 {
        let diff = self.duration.checked_sub(self.time_elapsed_in_millis);
        match diff {
            None => 0,
            Some(value) => value / 1000,
        }
    }

    fn is_finished(&self) -> bool {
        self.time_elapsed_in_millis >= self.duration
    }
}

pub struct Session {
    completed_tomatoes: u64,
    current_phase: Phase,
    timer_state: TimerState,
    on: bool,
}

impl Session {
    pub fn new() -> Session {
        Session {
            completed_tomatoes: 0,
            current_phase: Phase::Tomato,
            timer_state: TimerState::new(Phase::Tomato),
            on: true,
        }
    }

    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn is_paused(&self) -> bool {
        self.timer_state.paused
    }

    pub fn quit(&mut self) {
        self.on = false;
    }

    pub fn get_completed_tomatoes(&self) -> u64 {
        return self.completed_tomatoes;
    }

    pub fn get_current_phase(&self) -> Phase {
        return self.current_phase.clone();
    }

    pub fn update(&mut self) {
        if self.timer_state.is_finished() {
            self.handle_finished_phase();
        } else {
            self.timer_state = self.timer_state.update();
        }
    }

    fn handle_finished_phase(&mut self) {
        match self.current_phase {
            Phase::Tomato => {
                self.completed_tomatoes = self.completed_tomatoes + 1;
                self.current_phase = Phase::Break;
                self.timer_state = TimerState::new(Phase::Break);
                self.notify("Start a break?");
            }
            Phase::Break => {
                self.current_phase = Phase::Tomato;
                self.timer_state = TimerState::new(Phase::Tomato);
                self.notify("Start working?");
            }
        }
    }

    fn notify(&mut self, message: &str) {
        match Notification::new()
            .summary("Pomoxide Timer")
            .body(message)
            .show()
        {
            Ok(_msg) => {}
            Err(_err) => {
                println!("Error in displaying message");
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.timer_state = self.timer_state.toggle_pause();
    }

    pub fn reset(&mut self) {
        self.timer_state = TimerState::new(self.current_phase.clone());
    }

    pub fn skip(&mut self) {
        self.timer_state = match self.current_phase {
            Phase::Tomato => TimerState::new(Phase::Break),
            Phase::Break => TimerState::new(Phase::Tomato),
        };
        self.current_phase = match self.current_phase {
            Phase::Tomato => Phase::Break,
            Phase::Break => Phase::Tomato,
        };
    }
    pub fn get_time_remaining(&self) -> u128 {
        self.timer_state.get_time_remaining()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_timerstate_new() {
        let br = TimerState::new(Phase::Break);
        assert_eq!(5 * 60 * 1000, br.duration);
        let tomato = TimerState::new(Phase::Tomato);
        assert_eq!(25 * 60 * 1000, tomato.duration);
    }

    #[test]
    fn test_timerstate_pause_and_resume() {
        let mut state = TimerState::new(Phase::Tomato);
        assert_eq!(true, state.paused);
        state = state.resume();
        assert_eq!(false, state.paused);
        state = state.pause();
        assert_eq!(true, state.paused);
    }

    #[test]
    fn test_session_notify() {
        let mut session = Session::new();
        session.notify("Test notification");
    }
}
