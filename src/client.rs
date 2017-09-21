use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::{io, str};

use clap::{Arg, App};
use env_logger;
use futures::{self, future, Future};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_core::reactor::Handle;
use tokio_proto::TcpClient;
use tokio_proto::pipeline::ClientService;
use tokio_service::Service;

use minihttp::{self, Http};

pub struct Client {
    inner: ClientService<TcpStream, Http>,
}

impl Service for Client {
    type Request = minihttp::Request;
    type Response = minihttp::Response;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = io::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let resp = self.inner.call(req).and_then(|response| {
            info!("Sup: {:?}", response);
            future::ok(response)
        });
        Box::new(resp)
        //Box::new(self.inner.call(req).and_then(|response| {
        //    info!("!HEHI");
        //    future::ok(response)
        //}))
    }

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

    pub fn healthcheck(&self) -> Box<Future<Item = minihttp::Response, Error = io::Error>>  {
        let request = minihttp::Request {
            method: "GET".to_string(),
            path: "/healthcheck".to_string(),
            version: 1,
            headers: vec![
                ("User-Agent".to_string(), "rust".to_string()),
                ("Accepts".to_string(), "text/html".to_string()),
            ],
            body: "".to_string(),
        };
        info!("fjlaksdflaks");

        self.call(request)
        //let resp = Box::new(self.call(request).and_then(|resp| {
        //    info!("What is {:?}", resp);
        //    future::ok(resp)
        //}));

        //resp
    }
}
