use std::{
    sync::{Arc, Condvar, Mutex},
    task::Wake,
};

enum State {
    Empty,
    Waiting,
    Notified,
}
pub struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}
impl Signal {
    pub fn new() -> Self {
        Signal {
            state: Mutex::new(State::Empty),
            cond: Condvar::new(),
        }
    }
    pub fn wait(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => *state = State::Empty,
            State::Waiting => {
                panic!("multiple wait");
            }
            State::Empty => {
                *state = State::Waiting;
                while let State::Waiting = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
        }
    }
    pub fn notify(&self) {
        let mut state = self.state.lock().unwrap();
        match *state {
            State::Notified => {}
            State::Empty => *state = State::Notified,
            State::Waiting => {
                *state = State::Empty;
                self.cond.notify_one();
            }
        }
    }
}
impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}
