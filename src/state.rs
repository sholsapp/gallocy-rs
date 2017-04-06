use std::sync::Arc;
use std::sync::Condvar;
use std::time::Duration;

use timer;

pub enum RaftState {
    FOLLOWER,
    LEADER,
    CANDIDATE,
}

pub struct State {
    current_term: u64,
    commit_index: u64,
    last_applied: u64,
    voted_for: String, 
    state: RaftState,
    log: String,
    timer: timer::Timer,
}

// TODO(sholsapp): When "associated const" or equivalent lands in stable, or
// when we're allowed to use calls in a constant declaration, we should fix
// this so that type is std::time::Duration instead of u64.
const FOLLOWER_STEP: u64 = 2000;
const FOLLOWER_JITTER: u64 = 500;
const LEADER_STEP: u64 = 500;
const LEADER_JITTER: u64 = 0;

impl State {
    pub fn new() -> State {
        let cv = Arc::new(Condvar::new());
        State {
            current_term: 0,
            commit_index: 0,
            last_applied: 0,
            voted_for: "".to_owned(),
            state: RaftState::FOLLOWER,
            log: "".to_owned(),
            timer: timer::Timer::new(
                Duration::from_millis(FOLLOWER_STEP),
                Duration::from_millis(FOLLOWER_JITTER),
                cv), 
        }
    }

    pub fn get_current_term(&self) -> u64 {
        self.current_term
    }

    pub fn set_current_term(&mut self, term: u64) -> u64 {
        self.current_term = term;
        self.current_term
    }

    pub fn get_commit_index(&self) -> u64 {
        self.commit_index
    }

    pub fn set_commit_index(&mut self, index: u64) -> u64 {
        self.commit_index = index;
        self.commit_index
    }

    pub fn get_last_applied(&self) -> u64 {
        self.last_applied
    }

    pub fn set_last_applied(&mut self, index: u64) -> u64 {
        self.last_applied = index;
        self.last_applied
    }
}