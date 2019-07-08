
use std::process::{Command, Stdio};
use std::fs;
use std::fs::File;
use std::io::{Write, Read};

use base64::decode;
use std::sync::mpsc::{Receiver};

//Utilising same level module
use crate::benchy::robjs::SubmissionRequest;
use crate::db::conn;

pub struct Runner {
    prepare_cmd: Option<String>,
    execute_cmd: String,
    cleanup_cmd: Option<String>,
    path: String,
}

impl Runner {

    ///
    /// Creates a new Runner object for running submissions
    pub fn new(prepare_cmd: Option<String>,
        execute_cmd: String,
        cleanup_cmd: Option<String>,
        path: String) -> Runner {
            Runner {
                prepare_cmd,
                execute_cmd,
                cleanup_cmd,
                path
            }
    }

    ///
    /// run_cmd will take a command and TODO: arguments
    /// to execute the current program
    /// It will latch onto the stdout and use it for processing
    /// TODO: time the execute
    pub fn try_run(command: &Option<String>) {
        //NOTE: Unix only ATM, create variant for Windows later
        if let Some(ref s) = command {
            eprintln!("{}", s);
            Command::new("bash")
                .arg("-c")
                .arg(s)
                .output()
                .expect("Failed to execute the command given.");
        }
    }

    pub fn run_cmd_extract_output(command: &str) -> String {
        let mut output = String::new();
        let proc = Command::new("bash")
                        .arg("-c")
                        .arg(command)
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

        match proc.stdout.unwrap().read_to_string(&mut output) {
            Err(_) => { eprintln!("Unable to read stdout from process"); },
            Ok(_) => { println!("Process contents read"); }
        }
        output
    }

    ///
    /// Once the runner has been set up it will be able to
    /// execute the commands and benchmark
    pub fn run(&self, sub: SubmissionRequest) -> std::io::Result<()> {
        //TODO: Use SubmissionRequest with run_cmd
        let data = decode(&sub.data).unwrap();
        //Create a temporary file
        //TODO: Pipe data into Command using Stdio::piped();
        {
            let mut dump = File::create("./tmp.zip")?;
            dump.write_all(&data)?;
            dump.sync_all()?;
        }

        //Executes an unzip operation on a piped file: TODO: Replace with zip crate
        Command::new("unzip")
            .arg("./tmp.zip")
            .arg("-d")
            .arg(format!("{}/{}", self.path, sub.ident))
            .spawn()
            .expect("Failed to execute the command given.");

        match fs::remove_file("./tmp.zip") {
            Ok(_) => { println!("tmp.zip successfully removed"); },
            Err(_) => { eprintln!("tmp.zip was not removed"); }
        }
        Runner::try_run(&self.prepare_cmd);
        let data = Runner::run_cmd_extract_output(self.execute_cmd.as_ref());
        Runner::try_run(&self.cleanup_cmd);

        self.save_results(&sub.ident, &data);
        Ok(())
    }


    /// Receive method will listen to the rx channel
    /// Once object has been received it will attempt to execute
    /// the submission against the binded commands
    pub fn receive(&mut self, rx: Receiver<SubmissionRequest>) {
        loop {
            match rx.recv() {
                Ok(req) => {
                    eprintln!("{}", req.ident);
                    match self.run(req) {
                        Ok(_) => println!("Runner Executed Successfully"),
                        _ => println!("Runner failed to execute")
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
        }
    }


    /// Saves the result after the test has been executed
    /// It will call the db connection class to insert a submission
    fn save_results(&self, ident: &String, data: &String) {
        println!("Submission: {}--{}", ident, data);
        match conn::insert_sub(&conn::establish(), ident, data) {
            Ok(v) => { println!("Result sent, return value: {}", v); },
            Err(e) => { eprintln!("Unable to insert submission results: {}", e); }
        }
    }

}
