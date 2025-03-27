use clap::{Parser, Subcommand};
use colored::Colorize;
use std::{
    error::Error,
    io::{self, Write},
};

#[derive(Parser, Debug)]
#[command(name = "", version = "1.0", about = "Broker Client CLI")]
struct Input {
    #[command(subcommand)]
    category: Category,
}

#[derive(Subcommand, Debug)]
enum Category {
    /// Run a compose command
    Compose {
        #[command(subcommand)]
        command: ComposeCommand,
    },
}

// Subcommands specific to 'compose'
#[derive(Subcommand, Debug)]
enum ComposeCommand {
    /// Start the specified project <index or name>
    Start { name_or_index: String },
    /// List the available projects
    List,
}

const PROJECTS: [&str; 2] = ["compose", "another"];

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        let line = buffer.trim();

        if line == "exit" {
            return Ok(());
        }
        if line.is_empty() {
            continue;
        }

        let input = std::iter::once("").chain(line.split_whitespace());

        match Input::try_parse_from(input) {
            Ok(input) => match input.category {
                Category::Compose { command } => handle_compose(command),
            },
            Err(err) => {
                // Clap will format the error nicely
                err.print()?; // Print the clap error message to stderr
            }
        }
        println!();
    }
}

fn handle_compose(command: ComposeCommand) {
    match command {
        ComposeCommand::List => {
            println!("{}", "Projects".bold().underline());
            for (index, project) in PROJECTS.iter().enumerate() {
                println!("{} - {}", index, project)
            }
        }
        ComposeCommand::Start {
            name_or_index: project,
        } => {
            let index: Option<usize> = match project.parse::<usize>() {
                Ok(index) => {
                    if index < PROJECTS.len() {
                        Some(index)
                    } else {
                        println!("Index out of bounds.");
                        None
                    }
                }
                Err(_) => {
                    let index = PROJECTS
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case(&project));
                    if index.is_none() {
                        println!("Project '{}' is invalid.", project);
                    }
                    index
                }
            };
            if let Some(index) = index {
                println!("Starting '{}' ..", PROJECTS[index]);
            }
        }
    }
}
