#[macro_use] extern crate log;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate raft;
extern crate tokio_core;
extern crate hyper;

use clap::{Arg, App};
use hyper::Client;
use hyper::rt::{self, lazy, Future, Stream};


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

    // A runtime is needed to execute our asynchronous code.
    rt::run(lazy(move || {
        let client = Client::new();

        client
            // Make a GET /ip to 'http://httpbin.org'
            .get(format!("{}{}", addr, "/healthcheck").parse().unwrap())

            // And then, if the request gets a response...
            .and_then(|res| {
                println!("status: {}", res.status());

                // Concatenate the body stream into a single buffer...
                // This returns a new future, since we must stream body.
                res.into_body().concat2()
            })

            // And then, if reading the full body succeeds...
            .and_then(|body| {
                // The body is just bytes, but let's print a string...
                let s = ::std::str::from_utf8(&body).unwrap();

                println!("body: {}", s);

                // and_then requires we return a new Future, and it turns
                // out that Result is a Future that is ready immediately.
                Ok(())
            })

            // Map any errors that might have happened...
            .map_err(|err| {
                println!("error: {}", err);
            })
    }));
}
