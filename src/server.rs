use std::sync::Arc;

use rustc_serialize;
use futures::future::Future;
use hyper::header::ContentType;
use hyper::StatusCode;
use hyper::server::{Request, Response, Service};
use hyper::Chunk;
use futures::Stream;
use hyper;
use futures;

use state;
use messages::{HealthCheck};


/// State to share between tokio service instances.
///
pub struct StateService {
    // Shared state for implementing Raft consensus.
    pub state: Arc<state::State>,
}

/// Implement a Hyper service.
///
/// Implement a Hyper service, avoiding usage of self and &self to satisfy
/// Hyper's (and thus Tokio's and Future's) interfaces, which require 'static
/// lifetimes to work propertly with futures.
///
impl Service for StateService {

    // Service request type - see tokio.rs.
    type Request = Request;

    // Service response type - see tokio.rs.
    type Response = Response;

    // Service error type - see tokio.rs.
    type Error = hyper::Error;

    // Service future type - see tokio.rs.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    /// Handle a request.
    ///
    /// Handle a request by dispatching to the appropriate routes implemented
    /// by StateService and returning the value as a future.
    ///
    fn call(&self, req: Request) -> Self::Future {
        info!("{:?}", &req);

        let state = Arc::clone(&self.state);

        let path: String = req.path().to_owned();

        Box::new(req.body().concat2().and_then(move |body: Chunk| {

            let resp = match &*path {
                "/healthcheck" => StateService::health_check(state, body),
                "/raft/request_vote" => StateService::request_vote(state, body),
                //"/raft/append_entries" => self.not_found(&req),
                //"/raft/request" => self.not_found(&req),
                _ => StateService::not_found(state, body),
            };

            info!("{:?}", &resp);

            // Return a future to satisfy `and_then` trait.
            futures::future::ok(resp)
        }))
    }
}

impl StateService {

    /// The health check route handler.
    ///
    fn health_check(state: Arc<state::State>, _body: Chunk) -> Response {
        let state: &state::State = &*state;
        let msg = HealthCheck {
            message: "GOOD".to_owned(),
            current_term: state.get_current_term(),
            commit_index: state.get_commit_index(),
            last_applied: state.get_last_applied(),

        };

        let body = rustc_serialize::json::encode(&msg).unwrap(); 

        Response::new()
            .with_header(ContentType::json())
            .with_body(body)
    }

    /// The request vote route handler.
    ///
    fn request_vote(state: Arc<state::State>, body: Chunk) -> Response {
        let state: &state::State = &*state;

        let mut resp = Response::new();

        // TODO(sholsapp): What if this isn't UTF-8?
        let text = String::from_utf8(body.to_vec()).unwrap();

        if let Ok(json) = rustc_serialize::json::decode(&*text) as Result<HealthCheck, rustc_serialize::json::DecoderError> {
            // TODO(sholsapp): Implement me.. this is just dummy.
            resp.set_body(rustc_serialize::json::encode(&json).unwrap().into_bytes());
        } else {
            resp.set_status(StatusCode::InternalServerError);
        }

        resp

    }

    /// The not found route handler.
    ///
    fn not_found(_state: Arc<state::State>, _body: Chunk) -> Response {
        Response::new()
            .with_status(StatusCode::NotFound)
    }
}
