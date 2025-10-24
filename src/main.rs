use inquire::Select;
use std::collections::HashMap;
use std::fs;

fn main() {
    // Read and parse projects.json
    let data = fs::read_to_string("projects.json").expect("Failed to read projects.json");

    let projects: HashMap<String, Vec<String>> =
        serde_json::from_str(&data).expect("Failed to parse JSON");

    let project_names: Vec<_> = projects.keys().cloned().collect();

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
