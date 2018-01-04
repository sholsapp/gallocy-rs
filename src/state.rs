use std::sync::{Arc, Mutex, RwLock};
use std::sync::Condvar;
use std::time::Duration;

use timer;

#[derive(Copy,Clone,PartialEq)]
pub enum RaftState {
    FOLLOWER,
    LEADER,
    CANDIDATE,
}

#[derive(Clone)]
pub struct State {
    current_term: Arc<RwLock<u64>>,
    commit_index: Arc<RwLock<u64>>,
    last_applied: Arc<RwLock<u64>>,
    state: Arc<RwLock<RaftState>>,
    timer: Arc<Mutex<timer::Timer>>,
    timed_out: Arc<(Mutex<bool>, Condvar)>,
    // log: String,
    // voted_for: String,
    //lock: Arc<RwLock<bool>>,
    peers: Arc<RwLock<Vec<String>>>,
}

// TODO(sholsapp): When "associated const" or equivalent lands in stable, or
// when we're allowed to use calls in a constant declaration, we should fix
// this so that type is std::time::Duration instead of u64.
const FOLLOWER_STEP: u64 = 5000;
const FOLLOWER_JITTER: u64 = 1000;
const LEADER_STEP: u64 = 500;
const LEADER_JITTER: u64 = 0;


impl State {
    pub fn new() -> State {
        let timed_out = Arc::new((Mutex::new(false), Condvar::new()));
        State {
            current_term: Arc::new(RwLock::new(0)),
            commit_index: Arc::new(RwLock::new(0)),
            last_applied: Arc::new(RwLock::new(0)),
            state: Arc::new(RwLock::new(RaftState::FOLLOWER)),
            // voted_for: "".to_owned(),
            // log: "".to_owned(),
            timed_out: Arc::clone(&timed_out),
            timer: Arc::new(Mutex::new(timer::Timer::new(
                Duration::from_millis(FOLLOWER_STEP),
                Duration::from_millis(FOLLOWER_JITTER),
                Arc::clone(&timed_out),
            ))),
            //lock: Arc::new(RwLock::new(false)),
            peers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn get_current_term(&self) -> u64 {
        *self.current_term.read().unwrap()
    }

    pub fn set_current_term(&self, term: u64) -> u64 {
        let mut _current_term = self.current_term.write().unwrap();
        *_current_term = term;
        *_current_term
    }

    pub fn get_commit_index(&self) -> u64 {
        *self.commit_index.read().unwrap()
    }

    pub fn set_commit_index(&self, index: u64) -> u64 {
        let mut _commit_index = self.commit_index.write().unwrap();
        *_commit_index = index;
        *_commit_index
    }

    pub fn get_last_applied(&self) -> u64 {
        *self.last_applied.read().unwrap()
    }

    pub fn set_last_applied(&self, index: u64) -> u64 {
        let mut _last_applied = self.last_applied.write().unwrap();
        *_last_applied = index;
        *_last_applied
    }

    pub fn get_state(&self) -> RaftState {
        *self.state.read().unwrap()
    }

    pub fn set_state(&self, new_state: RaftState) -> RaftState {
        let mut _state = self.state.write().unwrap();
        *_state = new_state;
        *_state
    }

    /// Get the timer condition that indicates timeout.
    ///
    /// Any operations that depends on a signal that indicates timeout, e.g., for determining
    /// leader timeout, should use this condition.
    ///
    pub fn get_timer_cv(&self) -> Arc<(Mutex<bool>, Condvar)> {
        Arc::clone(&self.timed_out)
    }

    /// Start underlying workers.
    ///
    /// This method must be called before the object is useable.
    ///
    pub fn start(&self) {
        // Unwrap to indicate that we should never fail while holding the lock.
        self.timer.lock().unwrap().start();
    }

    /// Stop underlying workers.
    ///
    /// This method should be called before exiting the program.
    ///
    pub fn stop(&mut self) {
        // Unwrap to indicate that we should never fail while holding the lock.
        self.timer.lock().unwrap().stop();
    }

    pub fn add_peer(&self, peer: &String) {
        let mut list = self.peers.write().unwrap();
        (*list).push(peer.clone());
    }
}
