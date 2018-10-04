extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("td")
        .version("0.1.0")
        .author("sarna <sarna.dev@protonmail.com>")
        .about("A to-do list for the command line")
        .get_matches();
}
