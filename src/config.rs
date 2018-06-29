use std;

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct RuntimeConfiguration {
    pub master: bool,
    pub me: String,
    pub peers: std::vec::Vec<String>,
    pub port: u16,
}