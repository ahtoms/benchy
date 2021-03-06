

use std::sync::mpsc::Sender;
use actix_web::{Json, App, http::Method, Result};
use crate::benchy::robjs::{SubmissionRequest, SubmissionResponse};

///
/// Registers the routes for submit, passes App and a sender  channel so any
/// SubmissionRequest can be passed to the Runner
pub fn register_routes(app: App, tx: Sender<SubmissionRequest>) -> App {
    app.resource("/submit", move |r| {

        let t = tx; //We want to scope in tx for the closure
        r.method(Method::POST).with(move |req: Json<SubmissionRequest>| ->
            Result<Json<SubmissionResponse>> {
            let data = req.into_inner();
            match t.send(data) {
                Ok(_) => { Ok(Json(SubmissionResponse { result: 1 })) },
                Err(_) => { Ok(Json(SubmissionResponse { result: 0 })) }
            }

        })
    })
}


