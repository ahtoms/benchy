
use serde::{Deserialize, Serialize};
#[macro_use] extern crate serde_derive;
use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};


///BenchmarkInfo object which contains
///benchmark name and test names
#[derive(Deserialize, Serialize)]
pub struct BenchmarkInfo {
    name: String,
    tests: Vec<String>
}

impl Responder for BenchmarkInfo {
    type Item = HttpResponse;
    type Error = Error;

    fn response_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self);

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))

    }
}



fn benchmark_listing_route(_req: &HttpRequest) -> impl Responder {
    //What to put here?
    //Maybe change to database call? Rustqlite might be a good option

    //TODO: Replace object with function call to extra data
    BenchmarkInfo { name: "TestBenchy", test: Vec::new() } //Dummy
}

///
/// Registers the routes for submit, passes App and a sender  channel so any
/// SubmissionRequest can be passed to the Runner
pub fn register_routes(app: App, tx: Sender<SubmissionRequest>) -> App {
    app.resource("/tests", |r|
        r.method(http::Method::GET).f(benchmark_listing_route))
}


