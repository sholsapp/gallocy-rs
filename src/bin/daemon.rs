#[macro_use] extern crate log;
extern crate raft;
extern crate clap;
extern crate env_logger;
extern crate tokio_proto;
extern crate hyper;
extern crate rustc_serialize;

use std::thread;
use std::sync::Arc;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;

use clap::{Arg, App};

use raft::machine;
use raft::server;
use raft::state;
use raft::config;

use hyper::server::Http;


fn read_runtime_configuration() {
    let path = Path::new("/Users/sholsapp/workspace/gallocy-rs/config.json");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let text = &*s;

    if let Ok(json) = rustc_serialize::json::decode(text) as Result<config::RuntimeConfiguration, rustc_serialize::json::DecoderError> {
        // TODO(sholsapp): Implement me.. this is just dummy.
        println!("IT WORKED");
    } else {
        println!("NO WORK");
    }


}

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
    //drop(env_logger::init(object));

    read_runtime_configuration();


    let port = matches.value_of("port").unwrap_or("8080");
    let host = matches.value_of("host").unwrap_or("0.0.0.0");
    let addr = format!("{}:{}", host, port).parse().unwrap();
    let state = Arc::new(state::State::new());

    state.add_peer(&"10.0.0.2:8080".to_owned());
    state.add_peer(&"10.0.0.3:8080".to_owned());
    state.add_peer(&"10.0.0.4:8080".to_owned());

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
