extern crate actix_web;
extern crate serde_json;
extern crate futures;
extern crate base64;

macro_rules! defaults {
    (PORT) => (6776);
    (HOST) => ("127.0.0.1")
}

mod routes;
mod proc;
mod web;

use std::env;
use std::io::{self, BufRead};
use std::sync::mpsc::{channel, Receiver, Sender};

use actix_web::{server, App, fs};

use proc::runner::Runner;
use routes::submit::{SubmissionRequest};



/// Receive method will listen to the rx channel
/// TODO: Move to separate module
fn receive(runner: Runner, rx: Receiver<SubmissionRequest>) {
    loop {
        match rx.recv() {
            Ok(req) => {
                runner.run(req);
            },
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}

///
/// Retrieves the arguments from command line.
///
fn get_args() -> (u16, Option<String>) {
    let mut port : u16 = defaults!(PORT);
    let mut path = None;
    
    if let Some(ps) = env::args().nth(1) {
        match ps.parse::<u16>() {
            Ok(p) => { port = p; },
            _ => { eprintln!("Unable to parse port number") }
        }
    }
    if let Some(pth) = env::args().nth(2) {
        path = Some(pth);
    }

    (port, path)
}

///
/// Compiles the application object based on the register modules
/// TODO: Cleanup go() method and server initialisation
///
fn go(tx: Sender<SubmissionRequest>) -> App {
    let app = App::new();
    let app = app.handler("/static", fs::StaticFiles::new(".")
        .unwrap()
        .show_files_listing());
    let app = web::register_index(app);
    routes::submit::register_routes(app, tx)
}



fn main() {
    let (tx, rx): (Sender<SubmissionRequest>, Receiver<SubmissionRequest>) = channel();
    let (port, path) = get_args();
    server::new( move || {
        go(tx.clone()) //Need to clone the tx channel for closure so it maintains a copy
        //that isn't bound by main(). 
        //This is can be considered asynchronous.
    })
    .bind(format!("{}:{}", defaults!(HOST), port))
    .unwrap_or_else(|_| panic!("Cannot bind to port {}", port))
    .run();
    
    
    //Setup command line or read configuration for runner
    match path {
        Some(path) => {
            match Runner::try_from(path) {
                Some(runner) => {
                    receive(runner, rx);
                },
                None => { eprintln!("Failed to load config, check path supplied"); }
            }
        
        },
        None => {
            //Command line mode
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                //Handle input, TODO: Command pattern'd 
                //Here is a simple echo
                if let Ok(l) = line {
                    println!("Echo: {}", l)
                }
            }
        }
    }
}
