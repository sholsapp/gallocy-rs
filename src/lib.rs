#[macro_use] extern crate log;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate rustc_serialize;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use clap::{Arg, App};
use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct StatusService;

#[derive(RustcEncodable)]
struct Message {
    message: String,
}

impl Service for StatusService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;
    fn call(&self, _request: Request) -> Self::Future {
        let (code, message) = match _request.path() {
            "/200" => (200, "OK"),
            "/400" => (400, "Bad Request"),
            "/500" => (500, "Internal Server Error"),
            _ => (404, "Not Found")
        };
        let msg = Message {
            message: format!("{}", code),
        };
        let mut resp = Response::new();
        resp.header("Content-Type", "application/json");
        resp.status_code(code, message);
        resp.body(&rustc_serialize::json::encode(&msg).unwrap());
        future::ok(resp)
    }
}

pub fn main() {
    let matches = App::new("status-service")
                      .version("0.1.0")
                      .arg(Arg::with_name("port")
                           .short("p")
                           .long("port")
                           .value_name("PORT")
                           .help("Port to listen on."))
                      .get_matches();
    drop(env_logger::init());
    let port = matches.value_of("port").unwrap_or("8080");
    let host = "0.0.0.0";
    let addr = format!("{}:{}", host, port).parse().unwrap();
    info!("Serving on {}...", addr);
    TcpServer::new(Http, addr)
        .serve(|| Ok(StatusService));
}
