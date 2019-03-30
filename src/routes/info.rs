

use serde::{Deserialize, Serialize};
use serde_json::{Value};
use actix_web::{Json, App, http::Method, Result, HttpRequest, HttpResponse, http::StatusCode};

use crate::db::conn; //huh, 2018 edition is kind of neat


///BenchmarkInfo object which contains
///benchmark name and test names
#[derive(Serialize, Deserialize)]
pub struct BenchmarkInfo {
    pub name: String,
    pub tests: Vec<String>
}

pub fn get_submissions(_req: &HttpRequest) -> HttpResponse {
    let conn = conn::establish();
    let subs = conn::get_subs(&conn);


    return HttpResponse::build(StatusCode::OK).json(subs);
}

pub fn register_routes(app: App, test_data: BenchmarkInfo) -> App {
    let app = app.resource("/subs", |r| {
        r.method(Method::GET).f(get_submissions)
    });
    app.resource("/tests", move |r| {
        let t = test_data; //We are moving test_data to resource
        r.method(Method::GET).with(move |_: Json<Value>| -> Result<Json<BenchmarkInfo>> {
                Ok(Json(BenchmarkInfo { name: t.name.clone(), tests: t.tests.clone() }))
        });
    })
}



