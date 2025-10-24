use clap::{Parser, Subcommand};
use inquire::Select;
use inquire::Text;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
const FILE: &str = "projects.json";

/// Simple program to greet a person
#[derive(Parser)]
#[command(name = "vsm")]
#[command(about = "Visual Studio Code Source Control Manager", long_about = None)]
struct Args {
    /// Add a new project or subproject
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "add", about = "Add a new project or subproject")]
    Add(AddCommand),
}

// This must be a struct with #[command(subcommand)] for nested commands
#[derive(Parser)]
struct AddCommand {
    #[command(subcommand)]
    subcommand: AddSubcommands,
}

#[derive(Subcommand)]
enum AddSubcommands {
    Project,
    Subproject,
}

fn main() {
    let args = Args::parse();

    // Read and parse projects.json
    let data = fs::read_to_string(FILE).expect("Failed to read projects.json");

    let mut projects: HashMap<String, Vec<String>> =
        serde_json::from_str(&data).expect("Failed to parse JSON");

    let project_names: Vec<_> = projects.keys().cloned().collect();

    match args.command {
        Some(Commands::Add(add)) => match add.subcommand {
            AddSubcommands::Project => {
                let name: String = Text::new("Project name:").prompt().unwrap();
                projects.entry(name).or_default();
                save(&projects);
                println!("Project added.");
            }
            AddSubcommands::Subproject => {
                // if projects.is_empty() {
                //     println!("No projects exist. Add a project first.");
                //     return;
                // }
                // let project_names: Vec<_> = projects.keys().cloned().collect();
                // let project = Select::new("Select a project:", project_names)
                //     .prompt()
                //     .unwrap();
                // let sub_name: String = Text::new("Subproject name:").prompt().unwrap();
                // projects.entry(project).or_insert(vec![]).push(sub_name);
                // save(&projects);
                println!("Subproject added.");
            }
        },
        None => println!("No command provided."),
    }

    loop {
        let project = Select::new("Select a project:", project_names.clone()).prompt();

        let Ok(project) = project else { break };

        let mut subprojects = projects.get(&project).cloned().unwrap_or_default();

        subprojects.push("<- Back".to_string()); // fix type

        let subproject = Select::new("Select a subproject:", subprojects)
            .prompt()
            .unwrap();

        if subproject == "<- Back" {
            continue;
        }

        println!("You selected {project} -> {subproject}");
        break;
    }
}

fn save(projects: &HashMap<String, Vec<String>>) {
    let data = serde_json::to_string_pretty(projects).expect("Failed to serialize");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE)
        .expect("Failed to open file");
    file.write_all(data.as_bytes()).expect("Failed to write");
}
