use std::path::PathBuf;
use structopt::StructOpt;

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