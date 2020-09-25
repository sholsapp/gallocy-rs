#[macro_use] extern crate log;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate raft;
extern crate hyper;
extern crate tokio;

use clap::{Arg, App};
use hyper::{body, header, Body, Client, Method, Request, Response};
use std::sync::atomic;
use std::sync::Arc;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    info!("Remote address: {}", addr);

    // This is the size of the population, for which we'll generate an individual request per
    // member.
    let population: usize = 10;

    // This is the number indicating the majority of the population, for now that is just 50%+ of
    // the population.
    let quorum: usize = 5;

    let atomic_counter = Arc::new(atomic::AtomicUsize::new(0));

    // Prepare all the requests...
    let mut requests: Vec<Request<Body>> = Vec::new();
    for _ in 0..population {

        let request = Request::builder()
            .method(Method::GET)
            .uri(format!("http://{}{}", addr, "/healthcheck"))
            .header(header::USER_AGENT, "gallocy-rs/multiclient")
            .body(Body::empty())
            .unwrap();

        requests.push(request);
    }

    for request in requests.into_iter() {

        let _atomic_counter = Arc::clone(&atomic_counter);

        let client = Client::new();

        let response : Response<Body> = client.request(request).await?;

        let body: Vec<u8> = body::to_bytes(response.into_body()).await?.to_vec();

        let s = ::std::str::from_utf8(&body).unwrap();
        println!("Response: {}", s);

        _atomic_counter.fetch_add(1, atomic::Ordering::SeqCst);

    }

    while atomic_counter.load(atomic::Ordering::SeqCst) < quorum {
        print!(".");
    }

    println!("Quorum reached at {} responses.", atomic_counter.load(atomic::Ordering::SeqCst));

    Ok(())

}
