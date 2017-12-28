#[macro_use] extern crate log;
extern crate raft;
extern crate clap;
extern crate env_logger;
extern crate tokio_proto;
extern crate hyper;

use std::thread;
use std::sync::Arc;

use clap::{Arg, App};

use raft::machine;
use raft::server;
use raft::state;

use hyper::server::Http;

pub fn main() {
    let matches = App::new("server")
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
    let state = Arc::new(state::State::new());
    info!("Serving on {}...", addr);

    // Initialize the state object, start timers, etc.
    state.start();

    // Initialize the finite state machine struct.
    let machine_state = Arc::clone(&state);
    thread::spawn(move || {
        let machine = machine::Machine {
            state: machine_state,
        };
        machine.work();
    });

    let server = Http::new().bind(&addr, move || Ok(server::StateService {
        state: Arc::clone(&state),
    })).unwrap();

    server.run().unwrap();
}
