use std::sync::{Arc, Mutex};

use env_logger;
use clap::{Arg, App};
use tokio_proto::TcpClient;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_proto::pipeline::ClientService;
use std::net::SocketAddr;
use tokio_core::reactor::Handle;
use futures::{future, Future};
use std::{io, str};
use minihttp::Http;

pub struct Client {
    inner: ClientService<TcpStream, Http>,
}

impl Client {
    /// Establish a connection to a HTTP server at the provided `addr`.
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = Client, Error = io::Error>> {
        let ret = TcpClient::new(Http)
            .connect(addr, handle)
            .map(|client_service| {
                Client { inner: client_service }
            });
        Box::new(ret)
    }
}

pub fn main() {
    let matches = App::new("stated-client")
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
}