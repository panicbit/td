#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate ron;
extern crate serde;
extern crate clap;
extern crate dirs;

use clap::{App, Arg, SubCommand, ArgMatches};

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
        ).subcommand(
            SubCommand::with_name("list")
                .about("List all the tasks")
                .version(VERSION),
        ).subcommand(
            SubCommand::with_name("delete")
                .about("Delete a task")
                .version(VERSION)
                .arg(
                    Arg::with_name("task")
                        .index(1)
                        .help("Number of the task to be deleted"),
                ),
        ).get_matches();

    match matches.subcommand() {
        ("new", Some(matches)) => command_new(matches),
        ("list", Some(matches)) => command_list(matches),
        ("delete", Some(matches)) => command_delete(matches),
        _ => {},
    }
}

fn command_new(_matches: &ArgMatches) {
    let task = Task::new();

    match task.save() {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }
}

fn command_list(_matches: &ArgMatches) {
    match Task::list_all() {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }
}

fn command_delete(matches: &ArgMatches) {
    if let Some(task) = matches.value_of("task") {
        match Task::delete(task) {
            Ok(_) => (),
            Err(why) => println!("{}", why),
        }
    } else {
        println!("Which one? Try e.g. `td delete 1`");
    }
}
