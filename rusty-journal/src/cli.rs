use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::{File, OpenOptions};
use std::io::{BuffReader, Result, Seek, SeekFrom, Error, ErrorKind};

pub fn add_task(journal_path: PathBuf, task: Tasks) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    file.seek(SeekFrom::Start(0))?;

    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid task ID"));
    }

    tasks.remove(task_position - 1);

    file.seek(SeekFrom::Start(0));
    file.set_len(0);

    serde_json::to_writer(file, &tasks);

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Tasks is empty");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }
    Ok(())
}


fn collect_tasks(mut file: &File) -> Result<Vec<Tasks>> {
    file.seek(SeekFrom::Start(0));
    let mut tasks: Vec<Tasks> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0));
    Ok(tasks);
}

#[derive(Debug, StructOpt)]
pub enum Action {
    Add {
        #[structopt()]
        task: String
    },
    Done {
        #[structopt()]
        position: usize
    },
    List,
}

#[derive(Debug, StructOpt)]
#[structopt{
    name = "Rusty Journal",
    about = "a command-line tool to-do app written in Rust"
}]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}