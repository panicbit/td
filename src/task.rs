use std::fs::{create_dir, File};
use std::io::{self, Write};
use std::path::Path;

use dirs::data_local_dir;

use chrono::Local;

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
pub struct Task {
    task: String,
    outcome: String,
    desire: String,
}

pub fn new_task() -> Task {
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
    pub fn print(&self) {
        println!("{}", self.task);
        println!("in order to {}", self.outcome);
        println!("because I want to {}", self.desire);
    }

    pub fn save(&self) -> super::std::io::Result<()> {
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
        file.write_all(super::ron::ser::to_string(&self).unwrap().as_bytes())?;
        Ok(())
    }
}
