use std;

#[derive(RustcEncodable,RustcDecodable)]
pub struct Message {
    pub message: String,
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct RequestVote {
    pub commit_index: u64,
    pub term: u64,
    pub last_applied: u64,
    // sender: String,
}

#[derive(RustcEncodable,RustcDecodable)]
pub struct AppendEntries {
    pub leader_commit: u64,
    pub previous_log_index: u64,
    pub previous_log_term: u64,
    pub term: u64,
    pub entries: std::vec::Vec<String>,
    // sender: String,
}
