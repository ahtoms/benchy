extern crate actix_web;
extern crate serde_json;
extern crate futures;
extern crate base64;

mod routes;
mod proc;
mod db;
mod benchy;

use std::env;
use std::fs::File;
use std::io::{BufReader};
use std::sync::mpsc::{channel, Receiver, Sender};
use actix_web::{server, App, fs};
use proc::runner::Runner;
use crate::benchy::robjs::SubmissionRequest;
use crate::benchy::benchmark::{BenchyConfig, BenchmarkInfo};

macro_rules! defaults {
    (HOST) => ("127.0.0.1")
}

static HELP: &'static str = "
    To run benchy, specify a configuration file with the following properties:

    - name: String
    - port: Integer
    - root: String (path to serve html files)

    - prepare_cmd: String, Optional (script to prepare environment before each run),
    - execute_cmd: String (script to execute test (output will be extracted),
    - cleanup_cmd: String, Optional (script to clean up after test),
    - path: String (location that will be used)
    
    - tests: Array (String), Provides test information

";

///
/// Loads the configuration file specified in the command line
fn load_config() -> Result<BenchyConfig, &'static str> {
    if let Some(ps) = env::args().nth(1) {
        if ps == "--help" {
            Err(HELP)
        } else {
            match File::open(ps) {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    match serde_json::from_reader::<_, BenchyConfig>(reader) {
                        Ok(config) => Ok(config),
                        Err(_) => Err("Unable to parse config file")
                    }
                },
                Err(_) => Err("Unable to open file")
            }
        }
    } else {
        Err(HELP)
    }
}

///
/// Compiles the application object based on the register modules
fn go(tx: Sender<SubmissionRequest>, benchinfo: BenchmarkInfo) -> App {
    let app = App::new();
    let app = app.handler("/", fs::StaticFiles::new(benchinfo.root.to_string())
        .unwrap()
        .show_files_listing());
    let app = routes::info::register_routes(app, benchinfo);
    let app = routes::submit::register_routes(app, tx);
    app
}


fn main() {
    let (tx, rx): (Sender<SubmissionRequest>, Receiver<SubmissionRequest>) = channel();
    match load_config() {
        Ok(config) => {
            let port = config.port;
            let root = config.root;
            let name = config.name;
            let tests = config.tests;
            let mut runner = Runner::new(
                config.prepare_cmd,
                config.execute_cmd,
                config.cleanup_cmd,
                config.path,
            );
            server::new( move || {
                go(tx.clone(), BenchmarkInfo {
                    root: String::from(root.as_ref()),
                    name: String::from(name.as_ref()),
                    tests: Vec::from(tests.as_ref())
                })
            })
            .bind(format!("{}:{}", defaults!(HOST), port))
            .unwrap_or_else(|_| panic!("Cannot bind to port {}", port))
            .run();
            runner.receive(rx);
        },
        Err(e) => eprintln!("{}", e)
    }
}
