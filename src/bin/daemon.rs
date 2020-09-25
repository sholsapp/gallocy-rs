#[macro_use] extern crate log;
extern crate clap;
extern crate env_logger;
extern crate hyper;
extern crate raft;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::thread;

use clap::App;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

use raft::config;
use raft::machine;
use raft::server;
use raft::state;


fn read_runtime_configuration() -> config::RuntimeConfiguration {
    let config_path_key = "CONFIG";

    let mut config_path_value = "config.json".to_owned();
    if let Some(val) = env::var_os(config_path_key) {
        config_path_value = val.to_str().unwrap().to_owned();
    }

    let path = Path::new(&config_path_value);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.to_string()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.to_string()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let text = &*s;

    if let Ok(json) = rustc_serialize::json::decode(text)
            as Result<config::RuntimeConfiguration, rustc_serialize::json::DecoderError> {
        json
    } else {
        warn!("Failed to load configuration. Using default configuration.");
        config::RuntimeConfiguration {
            master: false,
            me: "127.0.0.1".to_owned(),
            peers: vec!(),
            port: 8080,
        }
    }


}

#[tokio::main]
pub async fn main() {

    env_logger::init();

    let _ = App::new("server")
                      .version("0.1.0")
                      .get_matches();

    let config = read_runtime_configuration();

    let addr = format!("{}:{}", config.me, config.port).parse().unwrap();

    let state = Arc::new(state::State::new());

    for peer in config.peers {
        state.add_peer(&peer.to_owned());
    }

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

    let server_state = Arc::clone(&state);

    let make_svc = make_service_fn(|_conn| {
        let handler_state = Arc::clone(&server_state);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                server::call(req, Arc::clone(&handler_state))
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("Error: {}", e);
    }

}
