#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dirs;
extern crate ron;
extern crate serde;

extern crate clap;
use clap::{App, Arg, SubCommand};

mod task;
use task::Task;

const VERSION: &str = "0.1.0";

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
        let task = Task::new();
        match task.save() {
            Ok(_) => (),
            Err(why) => println!("{}", why),
        }
    }
}
