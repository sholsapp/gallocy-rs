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

use std::io;

use clap::{Arg, App};
use futures::future;
use tokio_proto::TcpServer;
use tokio_service::Service;

pub mod state;
pub mod timer;
pub mod messages;

mod minihttp;
use minihttp::{Request, Response, Http};

use messages::{Message, RequestVote, AppendEntries};

struct StateService {
    state: state::State,
}

impl Service for StateService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;
    fn call(&self, req: Request) -> Self::Future {
        let resp = match req.path() {
            "/healthcheck" => self.health_check(&req),
            "/raft/request_vote" => self.request_vote(&req),
            "/raft/append_entries" => self.not_found(&req),
            "/raft/request" => self.not_found(&req),
            _ => self.not_found(&req),
        };
        info!("{} - {}", req.method(), req.path());
        for (k, v) in req.headers() {
            info!("\t > {}: {}", k, String::from_utf8(Vec::from(v)).unwrap());
        }
        info!("\t {}", req.body());
        future::ok(resp)
    }
}

impl StateService {

    fn health_check(&self, _: &Request) -> Response {
        let msg = Message {
            message: "GOOD".to_owned(),
        };
        let mut resp = Response::new();
        resp.header("Content-Type", "application/json");
        resp.status_code(200, "GOOD");
        resp.body(&rustc_serialize::json::encode(&msg).unwrap());
        resp
    }

    fn request_vote(&self, req: &Request) -> Response {
        info!("/raft/request_vote");
        let json: Message = rustc_serialize::json::decode(req.body()).unwrap();
        let mut resp = Response::new();
        resp.header("Content-Type", "application/json");
        resp.header("Connection", "close");
        resp.status_code(200, "GOOD");
        resp.body(&rustc_serialize::json::encode(&json).unwrap());
        resp
    }

    fn not_found(&self, _: &Request) -> Response {
        let mut resp = Response::new();
        resp.header("Content-Type", "text/html; charset=UTF-8");
        resp.status_code(404, "Not Found");
        resp.body("NOT FOUND");
        resp
    }
}

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
    let state = state::State::new();
    info!("Serving on {}...", addr);
    TcpServer::new(Http, addr)
        .serve(move || Ok(StateService {
            state: state.clone(),
        }));
}
