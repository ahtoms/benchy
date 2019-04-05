

use serde_json::{Value, json};
use actix_web::{Json, App, http::Method, Result, HttpRequest, HttpResponse, http::StatusCode};
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
    let app = app.resource("/subs", |r| {
        r.method(Method::GET).f(get_submissions)
    });
    app.resource("/tests", move |r| {
        let t = test_data; //We are moving test_data to resource
        r.method(Method::GET).with(move |_: Json<Value>| -> Result<Json<BenchmarkInfo>> {
                Ok(Json(BenchmarkInfo { root: t.root.clone(), name: t.name.clone(), tests: t.tests.clone() }))
        });
    })
}



