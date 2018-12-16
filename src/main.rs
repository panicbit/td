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

    if let Err(why) = task.save() {
        println!("{}", why);
    }
}

fn command_list(_matches: &ArgMatches) {
    if let Err(why) = Task::list_all() {
        println!("{}", why);
    }
}

fn command_delete(matches: &ArgMatches) {
    let task_n = match matches.value_of("task") {
        Some(task_n) => task_n,
        None => {
            println!("Which one? Try e.g. `td delete 1`");
            return;
        },
    };
    let task_n = parse_task_number(task_n);

    if let Err(why) = Task::delete(task_n) {
        println!("{}", why);
    }
}

fn parse_task_number(task_n: &str) -> usize {
    let task_n = task_n.trim().parse::<usize>()
        .unwrap_or_else(|e| panic!("{}", e));

    task_n
}
