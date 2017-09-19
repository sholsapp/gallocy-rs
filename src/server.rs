use std::sync::{Arc, Mutex};
use std::io;

use tokio_service::Service;
use rustc_serialize;
use futures::future;

use state;
use minihttp::{Request, Response};
use messages::{HealthCheck};


/// State to share between tokio service instances.
///
pub struct StateService {
    // Shared state for implementing Raft consensus.
    pub state: Arc<Mutex<state::State>>,
}

/// Implement a tokio service.
///
impl Service for StateService {

    // Service request type - see tokio.rs.
    type Request = Request;

    // Service response type - see tokio.rs.
    type Response = Response;

    // Service error type - see tokio.rs.
    type Error = io::Error;

    // Service future type - see tokio.rs.
    type Future = future::Ok<Response, io::Error>;

    /// Handle a request.
    ///
    /// Handle a request by dispatching to the appropriate routes implemented
    /// by StateService and returning the value as a future.
    ///
    fn call(&self, req: Request) -> Self::Future {
        info!("{:?}", req);
        let resp = match req.path() {
            "/healthcheck" => self.health_check(&req),
            "/raft/request_vote" => self.request_vote(&req),
            "/raft/append_entries" => self.not_found(&req),
            "/raft/request" => self.not_found(&req),
            _ => self.not_found(&req),
        };
        info!("{:?}", resp);
        future::ok(resp)
    }
}

impl StateService {

    /// The health check route handler.
    ///
    fn health_check(&self, _: &Request) -> Response {
        let state: &state::State = &self.state.lock().unwrap();
        let msg = HealthCheck {
            message: "GOOD".to_owned(),
            current_term: state.get_current_term(),
            commit_index: state.get_commit_index(),
            last_applied: state.get_last_applied(),

        };
        let mut resp = Response::new();
        resp.header("Content-Type", "application/json");
        resp.status_code(200, "GOOD");
        resp.body(&rustc_serialize::json::encode(&msg).unwrap());
        resp
    }

    /// The request vote route handler.
    ///
    fn request_vote(&self, req: &Request) -> Response {
        let mut resp = Response::new();
        if let Ok(json) = rustc_serialize::json::decode(req.body()) as Result<HealthCheck, rustc_serialize::json::DecoderError> {
            info!("current term: {}", self.state.lock().unwrap().get_current_term());
            resp.header("Content-Type", "application/json");
            resp.status_code(200, "OK");
            resp.body(&rustc_serialize::json::encode(&json).unwrap());
            resp
        } else {
            resp.status_code(500, "Internal Server Error");
            resp
        }

    }

    /// The not found route handler.
    ///
    fn not_found(&self, _: &Request) -> Response {
        let mut resp = Response::new();
        resp.header("Content-Type", "text/html; charset=UTF-8");
        resp.status_code(404, "Not Found");
        resp.body("NOT FOUND");
        resp
    }
}
