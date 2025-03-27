use dialoguer::{FuzzySelect, console::Term, theme::ColorfulTheme};

const PROJECTS: [&str; 2] = ["compose", "another"];
const COMMANDS: [&str; 3] = ["start", "stop", "list"];

struct ComposeStart {
    project: String,
}

fn main() {
    println!("Press `Ctrl+C` to exit");
    let _ = ctrlc::set_handler(move || {
        let _ = Term::stderr().show_cursor();
    });
    loop {
        let command = select_command();

        match command {
            "start" => {
                let project = select_project();
                println!("Starting {}", project);
            }
            "stop" => {
                let project = select_project();
                println!("Stopping {}", project);
            }
            "list" => {
                println!("Projects\n{:?}", PROJECTS);
            }
            _ => println!("invalid command"),
        }
        println!();
    }
}

fn select_command() -> &'static str {
    COMMANDS[FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a command")
        .items(&COMMANDS)
        .interact()
        .unwrap()]
}

fn select_project() -> &'static str {
    PROJECTS[FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a project")
        .items(&PROJECTS)
        .interact()
        .unwrap()]
}
