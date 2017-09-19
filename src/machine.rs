use std::sync::{Arc, Mutex, Condvar};

use state::{self, RaftState};

pub struct Machine {
    // Shared state for implementing Raft consensus.
    pub state: Arc<Mutex<state::State>>,
}

impl Machine {
    pub fn work(&self) {
        loop {
            // XXX: Implement me.
            let state: &state::State = &self.state.lock().unwrap();
            let timed_out: Arc<(Mutex<bool>, Condvar)> = state.get_timer_cv();
            //timer_cv.lock().unwrap();
            let &(ref lock, ref cv) = &*timed_out;

            match cv.wait(lock.lock().unwrap()) {
                Ok(_) => info!("GOT HERE"),
                Err(e) => info!("ERROR: {}", e),
            };
        }

    }

    fn state_follower(&self) -> RaftState {
        RaftState::FOLLOWER
    }

    fn state_leader(&self) -> RaftState {
        RaftState::LEADER
    }

    fn state_candidate(&self) -> RaftState {
        RaftState::CANDIDATE
    }
}
