
use std::process::Command;
use std::fs;

use serde::{Deserialize, Serialize};
use base64::decode;

//Utilising same level module
use crate::routes::submit::SubmissionRequest;


#[derive(Deserialize, Serialize)]
pub struct Runner {
    prepare_cmd: Option<String>,
    execute_cmd: String,
    cleanup_cmd: Option<String>,
    data_interp_cmd: Option<String>,
    path: String,
}

impl Runner {

    ///
    /// Creates a new Runner object for running submissions
    pub fn new(prepare_cmd: Option<String>,
        execute_cmd: String,
        cleanup_cmd: Option<String>,
        data_interp_cmd: Option<String>,
        path: String) -> Runner {
            Runner {
                prepare_cmd,
                execute_cmd,
                cleanup_cmd,
                data_interp_cmd,
                path
            }
    }
    
    ///
    /// Attempts to load the file and deserialize it using serde
    /// If it is unable to deserialise, return None,
    /// TODO: When TryFrom trait is in Stable, use trait
    pub fn try_from(path: String) -> Option<Self> {
        match fs::read_to_string(path) {
            Ok(contents) => {
                match serde_json::from_str::<Runner>(&contents) {
                    Ok(runner) => Some(runner),
                    Err(_) => None
                }        
            },
            Err(_) => None
        }
    }

    ///
    /// run_cmd will take a command and TODO: arguments
    /// to execute the current program
    /// It will latch onto the stdout and use it for processing
    /// TODO: time the execute
    pub fn run_cmd(command: &str) {
        //NOTE: Unix only ATM, create variant for Windows
        Command::new(command)
            .output()
            .expect("Failed to execute the command given.");
    }

    ///
    /// Once the runner has been set up it will be able to
    /// execute the commands and benchmark
    pub fn run(&self, sub: SubmissionRequest) {
        //TODO: Use SubmissionRequest with run_cmd
        let data = decode(sub.data);



        if let Some(ref s) = self.prepare_cmd {
            Runner::run_cmd(s);
        }
        Runner::run_cmd(self.execute_cmd.as_ref());
        if let Some(ref s) = self.cleanup_cmd {
            Runner::run_cmd(s);
        }
        if let Some(ref s) = self.data_interp_cmd {
            Runner::run_cmd(s);
        }
    }

}
