

use serde_json::{json};
use actix_web::{App, http::Method, HttpRequest, HttpResponse, http::StatusCode};
use crate::db::conn; //huh, 2018 edition is kind of neat
use crate::benchy::benchmark::BenchmarkInfo;

pub fn get_submissions(_req: &HttpRequest) -> HttpResponse {
    let conn = conn::establish();
    match conn::get_subs(&conn) {
        Ok(subs) =>  HttpResponse::build(StatusCode::OK).json(subs),
        Err(_) => HttpResponse::build(StatusCode::NO_CONTENT).json(json!({"status" : "db retrieval failure"}))
    }
}

pub fn register_routes(app: App, test_data: BenchmarkInfo) -> App {
    let app = app.resource("/submissions", |r| {
        r.method(Method::GET).f(get_submissions)
    });
    app.resource("/info", move |r| {
        let t = test_data; //We are moving test_data to resource
        r.method(Method::GET).f(move |_: &HttpRequest| -> HttpResponse {
                HttpResponse::build(StatusCode::OK).json(
                    BenchmarkInfo {
                        root: t.root.clone(),
                        index: t.index.clone(),
                        name: t.name.clone(),
                        tests: t.tests.clone()
                    }
                )
        });
    })
}

