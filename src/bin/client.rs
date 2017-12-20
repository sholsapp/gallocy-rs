#[macro_use] extern crate log;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate raft;
extern crate tokio_core;

use clap::{Arg, App};
use futures::{future, Future};
use tokio_core::reactor::Core;
use std::time;
use std::thread;

use raft::client;

pub fn main() {
    let matches = App::new("client")
        .version("0.1.0")
        .arg(Arg::with_name("host")
             .short("h")
             .long("host")
             .value_name("HOST")
             .help("Host to connect."))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .value_name("PORT")
             .help("Port to connect to."))
        .get_matches();
    drop(env_logger::init());
    let port: &str = matches.value_of("port").unwrap_or("8080");
    let host: &str = matches.value_of("host").unwrap_or("0.0.0.0");
    let addr: String = format!("{}:{}", host, port).parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let address = addr.parse().unwrap();
    let idk = core.run(
        client::Client::connect(&address, &handle).and_then(|client| {
            info!("Hi");
            client.healthcheck()
        })
    ).unwrap();

    info!("RES: {:?}", idk);

}