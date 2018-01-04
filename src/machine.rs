use std::sync::{Arc, Mutex, Condvar};

use state::{self, RaftState};
use messages::RequestVote;

pub struct Machine {
    // Shared state for implementing Raft consensus.
    pub state: Arc<state::State>,
}

impl Machine {
    pub fn work(&self) {
        loop {
            let state: &state::State = &*self.state;

            let timed_out: Arc<(Mutex<bool>, Condvar)> = state.get_timer_cv();
            let &(ref lock, ref cv) = &*timed_out;

            // Wait here forever *until* we're signalled from the state object's internal timer,
            // which indicates that a leader timeout has occurred.
            match cv.wait(lock.lock().unwrap()) {
                Ok(_) => {
                    if state.get_state() == RaftState::FOLLOWER {
                        state.set_state(RaftState::CANDIDATE);
                    }
                }
                Err(e) => error!("Something terrible has happened: {}", e),
            };

            match state.get_state() {
                RaftState::FOLLOWER => {
                    self.state_follower();
                },
                RaftState::LEADER => {
                    self.state_leader();
                },
                RaftState::CANDIDATE => {
                    self.state_candidate();
                }
            }
        }

    }

    fn state_follower(&self) -> RaftState {
        info!("I'm a follower.");
        RaftState::FOLLOWER
    }

    fn state_leader(&self) -> RaftState {
        info!("I'm a leader.");
        RaftState::LEADER
    }

    fn state_candidate(&self) -> RaftState {
        info!("I'm a candidate.");
        RaftState::CANDIDATE
    }
}
