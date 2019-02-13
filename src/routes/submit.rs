

use std::sync::mpsc::Sender;
use actix_web::{Json, App, http::Method, Result};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct SubmissionRequest{
    username: String,
    data: String,   
}

#[derive(Serialize)]
pub struct SubmissionResponse {
    result: u8,
}

pub fn register_routes(app: App, tx: Sender<SubmissionRequest>) -> App {
    app.resource("/submit", move |r| {
        let t = tx; //We want to scope in tx for the closure
        r.method(Method::POST).with(move |req: Json<SubmissionRequest>| -> Result<Json<SubmissionResponse>> {
            let data = req.into_inner();
            t.send(data).unwrap();
            Ok(Json(SubmissionResponse { result: 1 }))        
        })
    })
}


