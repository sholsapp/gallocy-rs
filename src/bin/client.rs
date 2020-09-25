#[macro_use] extern crate log;
extern crate clap;
extern crate env_logger;
extern crate futures;
extern crate raft;
extern crate hyper;
extern crate tokio;

use clap::{Arg, App};
use hyper::{body, Body, Client, Method, Request, Response};


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

    // A runtime is needed to execute our asynchronous code. Instead of using the following, we use
    // the #[tokio::main] macro on an async main function.
    //  let mut rt = tokio::runtime::Runtime::new().unwrap();
    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("http://{}{}", addr, "/healthcheck"))
        .body(Body::empty())
        .unwrap();

    let client = Client::new();

    let response : Response<Body> = client.request(request).await?;

    let body: Vec<u8> = body::to_bytes(response.into_body()).await?.to_vec();

    let s = ::std::str::from_utf8(&body).unwrap();
    println!("Response: {}", s);

    Ok(())

}
