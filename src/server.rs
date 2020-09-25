use std::sync::Arc;
use hyper::{body, Body, Request, Response, StatusCode, Method};
use rustc_serialize;

use crate::state;
use crate::messages::{HealthCheck};


type GenericError = Box<dyn std::error::Error + Send + Sync>;


/// Call the web server's request handler(s).
///
pub async fn call(req: Request<Body>, state: Arc<state::State>)
        -> Result<Response<Body>, GenericError> {

    info!("{:?}", &req);

    let state = Arc::clone(&state);

    let (parts, stream) = req.into_parts();

    let body: Vec<u8> = body::to_bytes(stream).await?.to_vec();

    let resp = match (parts.method, parts.uri.path()) {
        (Method::GET, "/healthcheck") => health_check(state, body),
        (Method::POST, "/raft/request_vote") => request_vote(state, body),
        //"/raft/append_entries" => self.not_found(&req),
        //"/raft/request" => self.not_found(&req),
        _ => not_found(state, body),
    };

    info!("{:?}", &resp);

    Ok(resp)
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
