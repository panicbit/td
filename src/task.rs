use std::fs::{create_dir_all, read_dir, remove_file, DirEntry, File};
use std::io::{self, Read, Result, Write};
use std::path::{Path, PathBuf};
use std::fmt;
use dirs::data_local_dir;
use chrono::Local;

// TODO: is the task done
// TODO: description (maybe)
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task: String,
    outcome: String,
    desire: String,
}

impl Task {
    pub fn new() -> Self {
        let task = ask("My task is to ");
        let task = first_letter_to_upper({task}.trim());

        let outcome = ask("in order to ");
        let outcome = {outcome}.trim().to_string();

        let desire = ask("because I want to ");
        let desire = {desire}.trim().to_string();

        Task { task, outcome, desire }
    }

    pub fn list_all() -> Result<()> {
        let td_path = td_path();

        if !td_path.exists() {
            println!("Seems like you haven't added any tasks yet.");
            println!("Try adding some with `td new`.");
            return Ok(());
        }

        for (n, entry) in read_dir(td_path)?.enumerate() {
            let entry = entry?;
            let mut f = File::open(entry.path()).unwrap_or_else(|e| panic!("{}", e));
            let mut contents = String::new();

            f.read_to_string(&mut contents)
                .unwrap_or_else(|e| panic!("{}", e));

            let task: Task = ::ron::de::from_str(&contents).unwrap_or_else(|e| panic!("{}", e));

            println!("{}. {}", n + 1, task);
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let td_path = td_path();

        create_dir_all(&td_path)?;

        let now = Local::now().to_rfc3339();
        let filename = format!("{} {}", now, &self.task);
        let task_path_string = td_path.join(filename); // that's atrocious
        let task_path = Path::new(&task_path_string);
        let mut file = File::create(&task_path)?;

        file.write_all(::ron::ser::to_string(&self).unwrap().as_bytes())?;

        Ok(())
    }

    pub fn delete(task_n: &str) -> Result<()> {
        let n: usize = task_n.trim().parse().unwrap_or_else(|e| panic!("{}", e));
        let td_path = td_path();

        if !td_path.exists() {
            println!("Seems like you haven't added any tasks yet.");
            println!("Try adding some with `td new`.");
            return Ok(());
        }

        let all_tasks: Vec<DirEntry> = read_dir(td_path)?.filter_map(Result::ok).collect();
        // TODO improve code below
        let target_task = &all_tasks.get(n - 1);
        let target_task = match target_task {
            Some(task) => task,
            None => {
                println!("Task not found.");
                return Ok(());
            }
        };

        remove_file(target_task.path())?;
        println!("Successfully removed the task.");

        Ok(())
    }
}

impl fmt::Display for Task {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}\nin order to {}\nbecause I want to {}\n",
            self.task, self.outcome, self.desire
        )
    }
}

fn ask(prompt: &str) -> String {
    print!("{}", prompt);

    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn first_letter_to_upper(s: &str) -> String {
    let mut chars = s.chars();

    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn td_path() -> PathBuf {
    let data_local_dir = match data_local_dir() {
        Some(dir) => dir,
        None => panic!("Could not open the local data directory"),
    };
    let td_path = Path::new(&data_local_dir).join("td");

    td_path
}
