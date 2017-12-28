use std;

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct HealthCheck {
    pub commit_index: u64,
    pub current_term: u64,
    pub last_applied: u64,
    pub message: String,
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct RequestVote {
    pub commit_index: u64,
    pub term: u64,
    pub last_applied: u64,
    // sender: String,
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct AppendEntries {
    pub leader_commit: u64,
    pub previous_log_index: u64,
    pub previous_log_term: u64,
    pub term: u64,
    pub entries: std::vec::Vec<String>,
    // sender: String,
}
