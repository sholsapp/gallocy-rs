#[macro_use] extern crate log;
extern crate bytes;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate httparse;
extern crate net2;
extern crate rustc_serialize;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod state;
pub mod timer;
pub mod messages;
pub mod minihttp;
pub mod server;
pub mod client;
pub mod machine;

use std::sync::{Arc, Mutex};

use clap::{Arg, App};
use tokio_proto::TcpServer;
use tokio_core::reactor::Core;

use minihttp::Http;

pub fn main() {
    let matches = App::new("stated")
                      .version("0.1.0")
                      .arg(Arg::with_name("host")
                           .short("h")
                           .long("host")
                           .value_name("HOST")
                           .help("Host to bind on."))
                      .arg(Arg::with_name("port")
                           .short("p")
                           .long("port")
                           .value_name("PORT")
                           .help("Port to listen on."))
                      .get_matches();
    drop(env_logger::init());
    let port = matches.value_of("port").unwrap_or("8080");
    let host = matches.value_of("host").unwrap_or("0.0.0.0");
    let addr = format!("{}:{}", host, port).parse().unwrap();
    let state = Arc::new(Mutex::new(state::State::new()));
    info!("Serving on {}...", addr);
    TcpServer::new(Http, addr)
        .serve(move || Ok(server::StateService {
            state: state.clone(),
        }));
}
