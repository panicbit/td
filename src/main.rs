use std::io::{self, Write};

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
        // task.print();
    }
}
