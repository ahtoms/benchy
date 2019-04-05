
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct BenchmarkSubmission {
    pub sub_id: i32,
    pub ident: String,
    pub data: String
}

///BenchmarkInfo object which contains
///benchmark name and test names
#[derive(Serialize, Deserialize)]
pub struct BenchmarkInfo {
    pub root: String,
    pub name: String,
    pub tests: Vec<String>
}

#[derive(Deserialize)]
pub struct BenchyConfig {
    pub name: String,
    pub root: String,
    pub port: u16,
    pub tests: Vec<String>,

    //runner variables
    pub prepare_cmd: Option<String>,
    pub execute_cmd: String,
    pub cleanup_cmd: Option<String>,
    pub path: String,
}



