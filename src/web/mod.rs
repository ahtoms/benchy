
use actix_web::{App, Result, HttpRequest, fs::NamedFile, http::Method};
use std::path::PathBuf;

///
///Returns the index page
///
fn index(_req: &HttpRequest) -> Result<NamedFile> {
    let f = PathBuf::from("./index.html");
    Ok(NamedFile::open(f)?)
}

///
/// Registers the index of the website
pub fn register_index(app: App) -> App {
    app.resource("/", |r| r.method(Method::GET).f(index))
}
