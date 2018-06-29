use std::sync::Arc;

use rustc_serialize;
use futures::future::Future;
use hyper::{Body, Request, Response, StatusCode, Chunk};
use hyper;
use futures::Stream;
use futures;

use state;
use messages::{HealthCheck};


/// Call the web server's request handler(s).
///
pub fn call(req: Request<Body>, state: Arc<state::State>)
        -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {

    info!("{:?}", &req);

    let state = Arc::clone(&state);

    let path: String = req.uri().path().to_owned();

    // Synchronously wait for and build an entire response body. WTF are these people thinking.. of
    // course we need the entire body to parse JSON.
    let body: Vec<u8> = req.into_body().map(|chunk| {
        chunk.iter().map(|byte| byte.clone()).collect::<Vec<u8>>()
    }).concat2().wait().unwrap();

    let resp = match &*path {
        "/healthcheck" => health_check(state, body),
        "/raft/request_vote" => request_vote(state, body),
        //"/raft/append_entries" => self.not_found(&req),
        //"/raft/request" => self.not_found(&req),
        _ => not_found(state, body),
    };

    info!("{:?}", &resp);

    // Return a future to satisfy `and_then` trait.
    Box::new(futures::future::ok(resp))
}

/// The health check route handler.
///
fn health_check(state: Arc<state::State>, _body: Vec<u8>) -> Response<Body> {
    let state: &state::State = &*state;
    let msg = HealthCheck {
        message: "GOOD".to_owned(),
        current_term: state.get_current_term(),
        commit_index: state.get_commit_index(),
        last_applied: state.get_last_applied(),

    };

    let body = rustc_serialize::json::encode(&msg).unwrap();

    Response::builder()
        .body(Body::from(body))
        .unwrap()
}

/// The request vote route handler.
///
fn request_vote(state: Arc<state::State>, body: Vec<u8>) -> Response<Body> {
    let state: &state::State = &*state;

    // TODO(sholsapp): What if this isn't UTF-8?
    let text = String::from_utf8(body).unwrap();

    if let Ok(json) = rustc_serialize::json::decode(&*text)
            as Result<HealthCheck, rustc_serialize::json::DecoderError> {

        Response::builder()
            .body(Body::from(rustc_serialize::json::encode(&json).unwrap().into_bytes()))
            .unwrap()

    } else {

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .unwrap()
    }

}

/// The not found route handler.
///
fn not_found(_state: Arc<state::State>, _body: Vec<u8>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
