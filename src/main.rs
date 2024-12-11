use std::{
    env,
    fs::{DirBuilder, DirEntry, File},
    os::unix::process::CommandExt,
    path::Path,
    process::{exit, Command as CMD},
};

const SAVES_DIR: &str = "/home/levi/.dot";
#[derive(Debug)]
enum Command {
    Init,
    Update,
    Add(String),
    Unknown,
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Init => {
                DirBuilder::new()
                    .create(SAVES_DIR)
                    .expect("The .dot directory already exists");
            }
            Command::Update => {
                //let mut commit_msg = String::new();
                // 1. Check whether save file exists.
                // 2. + continue
                // 3. - create file and add it to commit
                //let tracked_files = ["/home/levi/.zshrc", "/home/levi/.tmux.conf"];
                //for file in tracked_files.iter() {
                //    let file_name = file.split('/').last().expect("Incorrect file pattern ");
                //    let tracked_file = File::open(file).unwrap();
                //    let saved_file_path = format!("{}/{}", SAVES_DIR, file_name);
                //    let saved_file = match File::open(&saved_file_path) {
                //        Ok(saved_file) => {}
                //        Err(_) => {
                //            File::create_new("")
                //            CMD::new(format!("git add {saved_file_path}")).exec();
                //            let new_file_commit_message = format!("Added {file_name}; ");
                //            commit_msg.push_str(&new_file_commit_message);
                //        }
                //    };
                //}
            }
            Command::Unknown => {
                println!("Usage: dot <command>\n");
                exit(0)
            }
        }
    }
}

impl From<Vec<String>> for Command {
    fn from(args: Vec<String>) -> Self {
        let command = match args.get(1) {
            Some(arg) => arg,
            None => {
                println!("Usage");
                exit(0)
            }
        };
        match command {
            "init" => Command::Init,
            "update" => Command::Update,
            "add" => {}
            _ => Command::Unknown,
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.get(1) {
        let command = Command::from(arg.to_string());
        println!("{command:?}");
        command.run();
    } else {
        exit(1)
    }
}

//fn are_identical()
