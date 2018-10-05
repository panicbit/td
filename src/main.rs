use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;

#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde;

extern crate dirs;
use dirs::data_local_dir;

extern crate chrono;
use chrono::Local;

extern crate clap;
use clap::{App, Arg, SubCommand};

const VERSION: &str = "0.1.0";

fn first_letter_to_upper(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// TODO: is the task done
// TODO: description (maybe)
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    task: String,
    outcome: String,
    desire: String,
}

fn new_task() -> Task {
    print!("My task is to ");
    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");
    task = first_letter_to_upper(&task);
    task = task.trim().to_string();

    print!("in order to ");
    io::stdout().flush().unwrap();
    let mut outcome = String::new();
    io::stdin()
        .read_line(&mut outcome)
        .expect("Failed to read line");
    outcome = outcome.trim().to_string();

    print!("because I want to ");
    io::stdout().flush().unwrap();
    let mut desire = String::new();
    io::stdin()
        .read_line(&mut desire)
        .expect("Failed to read line");
    desire = desire.trim().to_string();

    Task {
        task: task,
        outcome: outcome,
        desire: desire,
    }
}

impl Task {
    fn print(&self) {
        println!("{}", self.task);
        println!("in order to {}", self.outcome);
        println!("because I want to {}", self.desire);
    }

    fn save(&self) -> std::io::Result<()> {
        let data_local_dir = match data_local_dir() {
            Some(dir) => dir,
            None => panic!("Could not open the local data directory"),
        };
        let td_path = Path::new(&data_local_dir).join("td");
        if !td_path.exists() {
            create_dir(&td_path)?;
        }
        let now = Local::now().to_rfc3339(); // TODO: make it nicer (maybe)
                                             // TODO: consider replacing spaces with something else
        let filename = format!("{} {}", now, &self.task);
        let task_path_string = td_path.join(filename); // that's atrocious
        let task_path = Path::new(&task_path_string);
        let mut file = File::create(&task_path)?;
        file.write_all(ron::ser::to_string(&self).unwrap().as_bytes())?;
        Ok(())
    }
}

fn main() {
    let matches = App::new("td")
        .version(VERSION)
        .author("sarna <sarna.dev@protonmail.com>")
        .about("A to-do list for the command line")
        .subcommand(
            SubCommand::with_name("new")
                .about("Add a new item to the list")
                .version(VERSION),
        ).get_matches();
    if let Some(matches) = matches.subcommand_matches("new") {
        let task = new_task();
        match task.save() {
            Ok(_) => (),
            Err(why) => println!("{}", why),
        }
    }
}
