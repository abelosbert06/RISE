use std::io::{stdout, stdin, Write};
use std::path::{self, Path};
use std::env::{self, current_dir};
use std::process::Command;
use std::fs::File;

fn main(){

    //command history (bug: does not write to the file) TODO
    /*
    let history_file_path = Path::new("history.txt");

    fn command_history_logger(command: &mut str, path: &Path){
        
        let mut history_file = match File::create(path){
            Ok(history_file) => history_file,

            Err(e) => {
                panic!("Could not initialise .history file! {}", e)

            },
        };

        match history_file.write_all(command.as_bytes()){
            Err(e) => panic!("Problem with the .history file {}", e),
            Ok(_) => {}
            //todo: create log file to log events
        }
    }
    */     

    loop {

        //displays current working directory
        match current_dir() {
        Ok(path) => {
            
            print!("[{}] 🦀 >> ", path.display());
            stdout().flush().unwrap();
        },

        Err(e) => println!("Failed to get current directory: {:?}", e),
    }

        

        let mut input = String::new();
        //command_history_logger(&mut input, history_file_path);
        stdin().read_line(&mut input).unwrap();

        let mut command = input.trim().split_whitespace();
        let program = command.next().unwrap();
        let arguments = command;

        match program {

            // cd command
            "cd" => {
                let directory = arguments.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(directory);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            
            // exit command
            "exit" => return,

            program => {
                let child_process = Command::new(program)
                    .args(arguments)
                    .spawn();

                //handles wrong input from user
                match child_process {
                    Ok(mut child_process) => { 
                        child_process.wait().unwrap(); 
                    },

                    Err(e) => {
                        eprintln!("{}", e)
                    },
                };
            }
        }
    }
}