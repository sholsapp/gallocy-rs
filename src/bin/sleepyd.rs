#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rand::{thread_rng, Rng};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::delay_for;

///
///
///
async fn sleepy_for(seconds: u64, _: Request<Body>) -> Result<Response<Body>, Infallible> {
    delay_for(Duration::from_secs(seconds)).await;
    Ok(Response::new(Body::from(format!("Slept for {} seconds!!", seconds))))
}

///
///
///
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let min : u64 = env::var("SLEEPY_MIN").map_or(0, |s| s.parse::<u64>().unwrap_or(0));
    let max : u64 = env::var("SLEEPY_MAX").map_or(10, |s| s.parse::<u64>().unwrap_or(10));

    info!("Configure sleepyd with SLEEPY_MIN and SLEEPY_MAX environment variables.");
    info!("CTRL-C to quit.");
    info!("     _                           _     ");
    info!("    | |                         | |    ");
    info!(" ___| | ___  ___ _ __  _   _  __| |    ");
    info!("/ __| |/ _ \\/ _ \\ '_ \\| | | |/ _` | ");
    info!("\\__ \\ |  __/  __/ |_) | |_| | (_| |  ");
    info!("|___/_|\\___|\\___| .__/ \\__, |\\__,_|");
    info!("                | |     __/ |          ");
    info!("                |_|    |___/           ");
    info!("                                       ");
    info!("Configuring sleepyd with SLEEPY_MIN={}, SLEEPY_MAX={}", min, max);

    let mut rng = thread_rng();
    // For every connection, we must make a `Service` to handle all incoming HTTP requests on said
    // connection.
    let make_svc = make_service_fn(|_conn| {
	let seconds: u64 = rng.gen_range(min, max);
        // This is the `Service` that will handle the connection.  `service_fn` is a helper to
        // convert a function that returns a Response into a `Service`.
        async move {
            let sleepy = move |request| sleepy_for(seconds, request);
            Ok::<_, Infallible>(service_fn(sleepy))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    info!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
